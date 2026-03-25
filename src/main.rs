use de::types::capabilities::Capabilities;
use de::types::supported_features::Cpu;
use de::types::virsh_domcapabilities::DomainCapabilities;
use k8s::K8sApi;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

mod cpu_caps;
mod de;
mod k8s;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let kubevirt_ns = "kubevirt".to_string();
    let selector = "kubevirt.io=virt-handler";
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let debugger_name = format!("debuggger-{}", timestamp);
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
    out_yaml(&mut io::sink())?;
    Ok(())
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

    let output = cpu_caps.to_yaml()?;
    if let Err(e) = out.write_all(output.as_bytes()) {
        return Err(Box::new(e));
    };
    Ok(())
}
