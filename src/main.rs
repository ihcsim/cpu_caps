use de::types::capabilities::Capabilities;
use de::types::supported_features::Cpu;
use de::types::virsh_domcapabilities::DomainCapabilities;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use k8s_openapi::api::core::v1::Pod;
use kube::{
    Client,
    api::{Api, ListParams},
};

mod cpu_caps;
mod de;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ns = "kubevirt";
    let selector = "kubevirt.io=virt-handler";

    let k8s_client = Client::try_default().await?;
    let pods: Api<Pod> = Api::namespaced(k8s_client, ns);
    let list_params = ListParams::default().labels(selector);
    for pod in pods.list(&list_params).await? {
        let status = match pod.status {
            Some(status) => status,
            None => continue,
        };
        if let Some(phase) = status.phase
            && phase != "Running"
        {
            continue;
        }

        let pod_name = match pod.metadata.name {
            Some(name) => name,
            None => continue,
        };

        let spec = match pod.spec {
            Some(spec) => spec,
            None => continue,
        };
        let node_name = match spec.node_name {
            Some(node_name) => node_name,
            None => continue,
        };
        println!(
            "Found virt-handler pod: {} on node: {}",
            pod_name, node_name
        );
    }

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
    Ok(())
}
