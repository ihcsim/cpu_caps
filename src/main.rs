use cpu_caps::types::LibvirtData;
use flate2::read::GzDecoder;
use k8s::K8sApi;
use log::info;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufReader, Cursor, Read};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tar::Archive;

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
    let virt_launcher_image = debugger_image.clone();
    let debugger_ttl_seconds = 3600;
    let src_path = Path::new("/var").join("lib").join("kubevirt-node-labeller");

    env_logger::init();
    let api = K8sApi::new(
        kubevirt_ns,
        src_path.clone(),
        selector,
        debugger_name,
        debugger_image,
        debugger_ttl_seconds,
    )
    .await?;
    let node_to_archive = api.extract_libvirt_data().await?;
    out_yaml(virt_launcher_image, node_to_archive, &mut io::stdout())?;
    Ok(())
}

fn out_yaml<W: io::Write>(
    virt_launcher_image: String,
    node_to_archive: HashMap<String, Box<dyn Read>>,
    out: &mut W,
) -> Result<(), Box<dyn Error>> {
    let mut libvirt_data: Vec<LibvirtData> = Vec::new();
    for (node_name, reader) in node_to_archive {
        info!("processing archive entry for node: {}", node_name);
        // read the archive entries into bufreader and then deserialize the XML
        // content
        let mut node_libvirt_data = LibvirtData::default();
        let decoder = GzDecoder::new(reader);
        let mut archive = Archive::new(decoder);
        for entry in archive.entries()? {
            let mut file = entry?;
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;
            let mut buf = BufReader::new(Cursor::new(contents));
            if let Some(file_name_raw) = file.path()?.file_name()
                && let Some(file_name) = file_name_raw.to_str()
            {
                info!("processing file: {}", file_name);
                match file_name {
                    "virsh_domcapabilities.xml" => {
                        node_libvirt_data.domcaps = de::from_reader(buf)?;
                    }
                    "capabilities.xml" => {
                        node_libvirt_data._caps = de::from_reader(buf)?;
                    }
                    "supported_features.xml" => {
                        node_libvirt_data.cpu = de::from_reader(buf)?;
                    }
                    ".version" => {
                        let mut virsh_version = String::new();
                        buf.read_to_string(&mut virsh_version)?;
                        node_libvirt_data.virsh_version =
                            virsh_version.trim_matches(char::is_whitespace).to_string();
                    }
                    _ => continue,
                };
            }
        }
        node_libvirt_data.node_name = node_name;
        node_libvirt_data.virt_launcher_image = virt_launcher_image.clone();
        libvirt_data.push(node_libvirt_data);
    }

    let cpu_caps = cpu_caps::compute(libvirt_data);
    let output = cpu_caps.to_yaml()?;
    if let Err(e) = out.write_all(output.as_bytes()) {
        return Err(Box::new(e));
    };
    Ok(())
}
