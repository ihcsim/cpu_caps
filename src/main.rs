use de::types::capabilities::Capabilities;
use de::types::supported_features::Cpu;
use de::types::virsh_domcapabilities::DomainCapabilities;
use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::core::v1::Pod;
use kube::{
    Client,
    api::{Api, AttachParams, ListParams, ObjectList, Patch, PatchParams, WatchEvent, WatchParams},
};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::{Path, PathBuf};
use tokio_util::io::ReaderStream;

mod cpu_caps;
mod de;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let kubevirt_ns = "kubevirt".to_string();
    let selector = "kubevirt.io=virt-handler";
    let debugger_name = "virt-handler-debugger4";
    let debugger_image = "quay.io/kubevirt/virt-launcher:v1.7.1".to_string();
    let debugger_ttl_seconds = 3600;
    let src_path = Path::new("/var").join("lib").join("kubevirt-node-labeller");

    let api = K8sApi::new(
        kubevirt_ns,
        src_path.clone(),
        selector,
        debugger_name,
        debugger_image,
        debugger_ttl_seconds,
    )
    .await?;
    api.inject_debuggers().await?;
    api.wait_for_debuggers().await?;
    api.copy_from_debuggers().await?;
    out_yaml(&mut io::stdout())?;
    Ok(())
}

struct K8sApi<'a> {
    pods: Api<Pod>,
    selector: &'a str,
    src_path: PathBuf,

    debugger_name: &'a str,
    debugger_image: String,
    debugger_ttl_seconds: u64,
}

impl<'a> K8sApi<'a> {
    async fn new(
        ns: String,
        src_path: PathBuf,
        selector: &'a str,
        debugger_name: &'a str,
        debugger_image: String,
        debugger_ttl_seconds: u64,
    ) -> Result<K8sApi<'a>, Box<dyn Error>> {
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

    async fn inject_debuggers(&self) -> Result<(), Box<dyn Error>> {
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

        let virt_handler_pods = self.list_virt_handler_pods().await?;
        for pod in virt_handler_pods {
            let phase = match &pod.status {
                Some(status) => status.phase.as_deref().unwrap_or(""),
                None => continue,
            };
            if phase != "Running" {
                continue;
            }

            let pod_name = pod.metadata.name.as_deref().unwrap_or("");
            let node_name = match &pod.spec {
                Some(spec) => spec.node_name.as_deref().unwrap_or(""),
                None => continue,
            };
            if pod_name.is_empty() || node_name.is_empty() {
                println!("skipping pod with missing name or node");
                continue;
            }

            println!("patching: pod {}, node {}", pod_name, node_name);
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
                println!("failed to patch pod {}: {}", pod_name, e);
                return Err(Box::new(e));
            };
            println!(
                "patched: pod {}, node {}, container {}",
                pod_name, node_name, self.debugger_name
            );
        }
        Ok(())
    }

    async fn wait_for_debuggers(&self) -> Result<(), Box<dyn Error>> {
        let virt_handler_pods = self.list_virt_handler_pods().await?;
        let total_pods = virt_handler_pods.items.len() as u32;
        let mut total_waits = 0;

        let watch_params = WatchParams::default().labels(self.selector).timeout(180);
        let mut stream = self.pods.watch(&watch_params, "0").await?.boxed();
        while let Some(status) = stream.try_next().await? {
            match status {
                WatchEvent::Modified(s) => {
                    let pod_name = s.metadata.name.as_deref().unwrap_or("");
                    println!("waiting: pod {} container {}", pod_name, self.debugger_name);

                    if let Some(status) = s.status
                        && let Some(ephemeral_containers_status) =
                            status.ephemeral_container_statuses
                        && ephemeral_containers_status.iter().any(|container| {
                            container.name == self.debugger_name
                                && container.state.as_ref().unwrap().running.is_some()
                        })
                    {
                        let attach_params = AttachParams::default()
                            .container(self.debugger_name)
                            .stdout(true)
                            .stderr(true);
                        let exec_cmds = vec![
                            "/bin/bash",
                            "-c",
                            "while [ ! -f ${ROOT_PATH}/.done ]; do sleep 1; done",
                        ];
                        let _attached = self.pods.exec(pod_name, exec_cmds, &attach_params).await?;
                        println!(
                            "completed: pod {} container {}",
                            pod_name, self.debugger_name
                        );

                        total_waits += 1;
                        if total_waits >= total_pods {
                            println!("all debug containers are ready");
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

    async fn copy_from_debuggers(&self) -> Result<(), Box<dyn Error>> {
        let virt_handler_pods = self.list_virt_handler_pods().await?;
        for pod in virt_handler_pods {
            let pod_name = pod.metadata.name.as_deref().unwrap_or("");
            let src_path = self.src_path.to_str().unwrap_or("");
            println!(
                "attaching: pod {}, containar {}, path {} ",
                pod_name, self.debugger_name, src_path
            );
            let exec_cmds = vec!["tar", "czvf", "output.tar.gz", src_path];

            let attach_params = AttachParams::default()
                .container(self.debugger_name)
                .stdout(true)
                .stderr(true);
            let mut attached = self.pods.exec(pod_name, exec_cmds, &attach_params).await?;
            if attached.stdout().is_none() {
                println!("no stdout output for pod {}", pod_name);
                continue;
            }

            if let Some(stderr) = attached.stderr()
                && let Some(result) = ReaderStream::new(stderr).next().await
            {
                if let Ok(bytes) = result {
                    println!("attached: stderr: {:?}", bytes);
                } else if let Err(e) = result {
                    return Err(Box::new(e));
                }
            }

            if let Some(stdout) = attached.stdout()
                && let Some(result) = ReaderStream::new(stdout).next().await
            {
                if let Ok(bytes) = result {
                    println!("attached: stdout: {:?}", bytes);
                    // println!("writing output to file {}", file_name.to_string_lossy());
                    // let mut file = File::create(file_name)?;
                    // file.write_all(&bytes)?;
                } else if let Err(e) = result {
                    return Err(Box::new(e));
                }
            }
        }
        Ok(())
    }

    async fn list_virt_handler_pods(&self) -> Result<ObjectList<Pod>, Box<dyn Error>> {
        let list_params = ListParams::default().labels(self.selector);
        let pods = self.pods.list(&list_params).await?;
        Ok(pods)
    }
}

fn out_yaml<W: io::Write>(out: &mut W) -> Result<(), Box<dyn Error>> {
    let path = Path::new("testdata").join("virsh_domcapabilities.xml");
    let xml_file = File::open(path)?;
    let buf = BufReader::new(xml_file);
    let domcaps: DomainCapabilities = de::from_reader(buf)?;

    let path = Path::new("testdata").join("capabilities.xml");
    let xml_file = File::open(path)?;
    let buf = BufReader::new(xml_file);
    let caps: Capabilities = de::from_reader(buf)?;

    let path = Path::new("testdata").join("supported_features.xml");
    let xml_file = File::open(path)?;
    let buf = BufReader::new(xml_file);
    let cpu: Cpu = de::from_reader(buf)?;

    let node_names = vec!["isim-dev"];
    let virsh_version = r#"Compiled against library: libvirt 11.0.0
Using library: libvirt 11.0.0
Using API: QEMU 11.0.0
"#;
    let virt_launcher_version = "1.6.3";
    let cpu_caps = cpu_caps::compute(
        node_names,
        &caps,
        &domcaps,
        &cpu,
        virsh_version,
        virt_launcher_version,
    );

    let _output = cpu_caps.to_yaml()?;
    // if let Err(e) = out.write_all(output.as_bytes()) {
    //     return Err(Box::new(e));
    // };
    Ok(())
}
