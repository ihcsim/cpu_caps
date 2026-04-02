use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::core::v1::Pod;
use kube::{
    Client,
    api::{Api, AttachParams, ListParams, ObjectList, Patch, PatchParams, WatchEvent, WatchParams},
};
use log::{debug, error, info};
use std::collections::HashMap;
use std::error::Error;
use std::io::{Cursor, Read};
use std::path::PathBuf;
use tokio_util::io::ReaderStream;

pub struct K8sApi {
    pods: Api<Pod>,
    selector: String,
    src_path: PathBuf,

    debugger_name: String,
    debugger_image: Option<String>,
    debugger_ttl_seconds: u64,
}

impl K8sApi {
    pub async fn new(
        ns: String,
        src_path: PathBuf,
        selector: String,
        debugger_name: String,
        debugger_image: Option<String>,
        debugger_ttl_seconds: u64,
    ) -> Result<K8sApi, Box<dyn Error>> {
        let client = Client::try_default().await?;
        let pods = Api::namespaced(client.clone(), ns.as_str());
        Ok(K8sApi {
            pods,
            selector,
            src_path,
            debugger_name,
            debugger_image,
            debugger_ttl_seconds,
        })
    }

    pub async fn exec_cp_libvirt_data(&mut self) -> Result<ExecOutput, Box<dyn Error>> {
        self.inject_debuggers().await?;
        self.wait_for_debuggers().await?;
        self.copy_from_debuggers().await
    }

    async fn inject_debuggers(&mut self) -> Result<(), Box<dyn Error>> {
        let virt_handler_pods = self.list_virt_handler_pods().await?;
        if virt_handler_pods.items.is_empty() {
            return Err("no virt-handler pods found".into());
        }

        // if the debugger image is not provided, use the image of the first virt-handler pod
        if self.debugger_image.is_none()
            && let Some(spec) = &virt_handler_pods.items[0].spec
            && let Some(image) = &spec.containers[0].image
            && let Some(pos) = image.find(":")
        {
            let tag = image.get(pos + 1..).map(|s| s.to_string());
            self.debugger_image = format!(
                "quay.io/kubevirt/virt-launcher:{}",
                tag.unwrap_or("latest".to_string())
            )
            .into();
        }
        info!(
            "using debugger image: {}",
            self.debugger_image.as_ref().unwrap()
        );

        let patch_ephemeral_containers = serde_json::json!({
            "spec": {
                "ephemeralContainers": [
                    {
                        "name": self.debugger_name,
                        "image": self.debugger_image,
                        "env": [
                            {
                                "name": "CONTAINER_TTL_SECONDS",
                                "value": self.debugger_ttl_seconds.to_string().as_str(),
                            },
                            {
                                "name": "ROOT_PATH",
                                "value": self.src_path.to_str(),
                            }
                        ],
                        "command": [
                            "/bin/bash",
                            "-c",
                            "set -xe
mkdir -p ${ROOT_PATH}
node-labeller.sh
virsh version > ${ROOT_PATH}/.version
touch ${ROOT_PATH}/.done
sleep ${CONTAINER_TTL_SECONDS:-3600}"
                        ],
                        "securityContext": {
                            "privileged": true
                        }
                    }
                ]
            }
        });

        for pod in virt_handler_pods {
            let pod_name = pod.metadata.name.as_deref().unwrap_or("");
            let phase = match &pod.status {
                Some(status) => status.phase.as_deref().unwrap_or(""),
                None => continue,
            };
            if phase != "Running" {
                debug!("skipping: pod: {} reason: phase {}", pod_name, phase);
                continue;
            }

            let node_name = match &pod.spec {
                Some(spec) => spec.node_name.as_deref().unwrap_or(""),
                None => continue,
            };
            if pod_name.is_empty() || node_name.is_empty() {
                debug!("skipping: pod with missing name or node. should not happen",);
                continue;
            }

            info!(
                "patching: pod {} node {} debugger {}",
                pod_name, node_name, self.debugger_name
            );
            let patch_params = PatchParams::default();
            if let Err(e) = self
                .pods
                .patch_ephemeral_containers(
                    pod_name,
                    &patch_params,
                    &Patch::Strategic(&patch_ephemeral_containers),
                )
                .await
            {
                error!("failed to patch pod {}: {}", pod_name, e);
                return Err(Box::new(e));
            };
        }
        Ok(())
    }

    async fn wait_for_debuggers(&self) -> Result<(), Box<dyn Error>> {
        let virt_handler_pods = self.list_virt_handler_pods().await?;
        let total_pods = virt_handler_pods.items.len() as u32;
        let mut total_waits = 0;

        let watch_params = WatchParams::default()
            .labels(self.selector.as_str())
            .timeout(180);
        let mut stream = self.pods.watch(&watch_params, "0").await?.boxed();
        while let Some(status) = stream.try_next().await? {
            match status {
                WatchEvent::Modified(s) => {
                    let pod_name = s.metadata.name.as_deref().unwrap_or("");
                    info!("waiting: pod {} container {}", pod_name, self.debugger_name);

                    if let Some(status) = s.status
                        && let Some(ephemeral_containers_status) =
                            status.ephemeral_container_statuses
                        && ephemeral_containers_status.iter().any(|container| {
                            container.name == self.debugger_name
                                && container.state.as_ref().unwrap().running.is_some()
                        })
                    {
                        let attach_params = AttachParams::default()
                            .container(&self.debugger_name)
                            .stdout(true)
                            .stderr(true);
                        let exec_cmds = vec![
                            "/bin/bash",
                            "-c",
                            r#"while [ ! -f ${ROOT_PATH}/.done ]; do echo "${ROOT_PATH}"/.done is not ready; sleep 1; done"#,
                        ];
                        let mut attached =
                            self.pods.exec(pod_name, exec_cmds, &attach_params).await?;
                        if let Some(stderr) = attached.stderr()
                            && let Some(result) = ReaderStream::new(stderr).next().await
                        {
                            if let Ok(bytes) = result {
                                info!("waiting: stderr: {:?}", bytes);
                            } else if let Err(e) = result {
                                return Err(Box::new(e));
                            }
                        }
                        if let Some(stderr) = attached.stdout()
                            && let Some(result) = ReaderStream::new(stderr).next().await
                        {
                            if let Ok(bytes) = result {
                                info!("waiting: stdout: {:?}", bytes);
                            } else if let Err(e) = result {
                                return Err(Box::new(e));
                            }
                        }

                        total_waits += 1;
                        if total_waits >= total_pods {
                            break;
                        }
                    }
                }
                WatchEvent::Error(e) => {
                    return Err(Box::new(e));
                }
                _ => continue,
            };
        }
        Ok(())
    }

    async fn copy_from_debuggers(&self) -> Result<ExecOutput, Box<dyn Error>> {
        let mut nodes_to_archive: HashMap<String, Box<dyn Read>> = HashMap::new();
        let virt_handler_pods = self.list_virt_handler_pods().await?;
        for pod in virt_handler_pods {
            let pod_name = pod.metadata.name.as_deref().unwrap_or("");
            let src_path = self.src_path.to_str().unwrap_or("");
            info!(
                "attaching: pod {}, debugger {}, path {} ",
                pod_name, self.debugger_name, src_path
            );

            // stream the archive content to stdout and then read the stdout of
            // the debugger container
            let tar_cmd = format!("tar -C {} -czO .", src_path);
            let exec_cmds = vec!["/bin/bash", "-c", tar_cmd.as_str()];
            let attach_params = AttachParams::default()
                .container(&self.debugger_name)
                .stdout(true)
                .stderr(true);
            let mut attached = self.pods.exec(pod_name, exec_cmds, &attach_params).await?;
            if let Some(stdout) = attached.stdout() {
                let mut stream = ReaderStream::new(stdout);
                let mut stream_contents = Vec::new();
                while let Some(chunk) = stream.next().await {
                    stream_contents.extend_from_slice(&chunk?);
                }
                let read = Cursor::new(stream_contents.clone());
                if let Some(spec) = pod.spec
                    && let Some(node_name) = spec.node_name
                {
                    nodes_to_archive.insert(node_name, Box::new(read));
                }
            }
            if let Some(stderr) = attached.stderr()
                && let Some(result) = ReaderStream::new(stderr).next().await
            {
                if let Ok(bytes) = result {
                    info!("stderr: {:?}", bytes);
                } else if let Err(e) = result {
                    return Err(Box::new(e));
                }
            }
        }

        Ok(ExecOutput {
            nodes_to_archive,
            virt_launcher_image: self.debugger_image.clone().unwrap_or_default(),
        })
    }

    async fn list_virt_handler_pods(&self) -> Result<ObjectList<Pod>, Box<dyn Error>> {
        let list_params = ListParams::default().labels(self.selector.as_str());
        let pods = self.pods.list(&list_params).await?;
        Ok(pods)
    }
}

pub struct ExecOutput {
    pub nodes_to_archive: HashMap<String, Box<dyn Read>>,
    pub virt_launcher_image: String,
}
