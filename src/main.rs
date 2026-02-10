use de::types::capabilities::Capabilities;
use de::types::supported_features::Cpu;
use de::types::virsh_domcapabilities::DomainCapabilities;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

mod de;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "testdata/virsh_domcapabilities.xml";
    let path = Path::new(filename);
    let xml_file = File::open(path)?;
    let buf = BufReader::new(xml_file);
    let _domcap_obj: DomainCapabilities = de::from_reader(buf)?;

    let filename = "testdata/capabilities.xml";
    let path = Path::new(filename);
    let xml_file = File::open(path)?;
    let buf = BufReader::new(xml_file);
    let _cap_obj: Capabilities = de::from_reader(buf)?;

    let filename = "testdata/supported_features.xml";
    let path = Path::new(filename);
    let xml_file = File::open(path)?;
    let buf = BufReader::new(xml_file);
    let _obj: Cpu = de::from_reader(buf)?;

    Ok(())
}
