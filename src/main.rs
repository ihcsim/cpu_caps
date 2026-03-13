use de::types::capabilities::Capabilities;
use de::types::supported_features::Cpu;
use de::types::virsh_domcapabilities::DomainCapabilities;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

mod cpu_caps;
mod de;

fn main() -> Result<(), Box<dyn Error>> {
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
    println!("{}", output);
    Ok(())
}
