use de::types::DomainCapabilities;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

mod de;

fn main() -> Result<(), Box<dyn Error>> {
    let filenames = vec!["testdata/virsh_domcapabilities.xml"];
    for filename in filenames {
        let path = Path::new(filename);
        let xml_file = File::open(path)?;
        let buf = BufReader::new(xml_file);
        let _obj: DomainCapabilities = de::from_reader(buf)?;
    }

    Ok(())
}
