use de::types::capabilities::Capabilities;
use de::types::supported_features::Cpu;
use de::types::virsh_domcapabilities::DomainCapabilities;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::path::Path;

use k8s_openapi::api::core::v1::Pod;
use kube::{
    Client,
    api::{Api, ListParams, Patch, PatchParams},
};

mod cpu_caps;
mod de;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ns = "kubevirt";
    let selector = "kubevirt.io=virt-handler";
    patch_virt_handler_with_debugger(ns, selector).await?;
    let buf = serialize_to_yaml()?;
    println!("{}", buf.get_ref().get_ref());
    Ok(())
}

async fn patch_virt_handler_with_debugger(
    namespace: &str,
    selector: &str,
) -> Result<(), Box<dyn Error>> {
    let patch_params = PatchParams::default();
    let patch_ephemeral_containers = serde_json::json!({
        "spec": {
            "ephemeralContainers": [
                {
                    "name": "virt-handler-debugger3",
                    "image": "busybox:latest",
                    "command": ["sleep", "3600"],
                    "securityContext": {
                        "privileged": true
                    }
                }
            ]
        }
    });

    let k8s_client = Client::try_default().await?;
    let pods: Api<Pod> = Api::namespaced(k8s_client, namespace);
    let list_params = ListParams::default().labels(selector);
    for pod in pods.list(&list_params).await? {
        let phase = match pod.status {
            Some(status) => status.phase.unwrap_or_else(|| "".to_string()),
            None => continue,
        };
        if phase != "Running" {
            continue;
        }

        let pod_name = pod.metadata.name.unwrap_or_else(|| "".to_string());
        let node_name = match pod.spec {
            Some(spec) => spec.node_name.unwrap_or_else(|| "".to_string()),
            None => continue,
        };
        if pod_name.is_empty() || node_name.is_empty() {
            println!("skipping pod with missing name or node");
            continue;
        }

        println!("patching pod {} on node {}", pod_name, node_name);
        if let Err(e) = pods
            .patch_ephemeral_containers(
                pod_name.as_str(),
                &patch_params,
                &Patch::Strategic(&patch_ephemeral_containers),
            )
            .await
        {
            println!("failed to patch pod {}: {}", pod_name, e);
            return Err(Box::new(e));
        };
    }

    Ok(())
}

fn serialize_to_yaml() -> Result<BufReader<Cursor<String>>, Box<dyn Error>> {
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

    let output = cpu_caps.to_yaml()?;
    let cursor = Cursor::new(output);
    Ok(BufReader::new(cursor))
}
