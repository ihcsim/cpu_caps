use de::types::capabilities::Capabilities;
use de::types::supported_features::Cpu;
use de::types::virsh_domcapabilities::DomainCapabilities;
use flate2::read::GzDecoder;
use k8s::K8sApi;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
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
    let out_files = api.copy_from_debuggers().await?;
    out_yaml(out_files, &mut io::sink())?;
    Ok(())
}

fn out_yaml<W: io::Write>(
    out_files: HashMap<String, Box<dyn Read>>,
    out: &mut W,
) -> Result<(), Box<dyn Error>> {
    for (node_name, reader) in out_files {
        let decoder = GzDecoder::new(reader);
        let mut archive = Archive::new(decoder);
        for entry in archive.entries()? {
            let mut file = entry?;
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;

            if let Ok(path) = file.path()
                && let Some(file_name_raw) = path.file_name()
                && let Some(file_name) = file_name_raw.to_str()
            {
                let mut buf = BufReader::new(Cursor::new(contents));
                match file_name {
                    "virsh_domcapabilities.xml" => {
                        let domcaps: DomainCapabilities = de::from_reader(buf)?;
                    }
                    "capabilities.xml" => {
                        let caps: Capabilities = de::from_reader(buf)?;
                    }
                    "supported_features.xml" => {
                        let cpu: Cpu = de::from_reader(buf)?;
                    }
                    ".version" => {
                        let mut virsh_version = String::new();
                        buf.read_to_string(&mut virsh_version)?;
                    }
                    _ => continue,
                };
            }
        }
    }

    //     let virt_launcher_version = "1.6.3";
    //     let cpu_caps = cpu_caps::compute(
    //         node_names,
    //         &caps,
    //         &domcaps,
    //         &cpu,
    //         virsh_version,
    //         virt_launcher_version,
    //     );
    //
    //     let output = cpu_caps.to_yaml()?;
    //     if let Err(e) = out.write_all(output.as_bytes()) {
    //         return Err(Box::new(e));
    //     };
    Ok(())
}
