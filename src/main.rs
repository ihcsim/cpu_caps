use clap::Parser;
use cpu_caps::types::LibvirtData;
use flate2::read::GzDecoder;
use k8s::K8sApi;
use log::info;
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
    let args = Cli::parse();
    let kubevirt_ns = args.kubevirt_ns;
    let selector = args.selector;
    let debugger_image = args.debugger_image;
    let debugger_ttl_seconds = args.debugger_ttl_seconds;

    let virt_launcher_image = debugger_image.clone();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let debugger_name = format!("debugger-{}", timestamp);
    let src_path = Path::new("/var").join("lib").join("kubevirt-node-labeller");

    env_logger::init();
    let api = K8sApi::new(
        kubevirt_ns,
        src_path,
        selector,
        debugger_name,
        debugger_image,
        debugger_ttl_seconds,
    )
    .await?;
    let exec_output = api.exec_cp_libvirt_data().await?;
    out_yaml(exec_output, &mut io::stdout())?;
    Ok(())
}

fn out_yaml<W: io::Write>(exec_output: k8s::ExecOutput, out: &mut W) -> Result<(), Box<dyn Error>> {
    let mut libvirt_data: Vec<LibvirtData> = Vec::new();
    for (node_name, reader) in exec_output.nodes_to_archive {
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
        node_libvirt_data.virt_launcher_image = exec_output.virt_launcher_image.clone();
        libvirt_data.push(node_libvirt_data);
    }

    let cpu_caps = cpu_caps::compute(libvirt_data);
    let output = cpu_caps.to_yaml()?;
    if let Err(e) = out.write_all(output.as_bytes()) {
        return Err(Box::new(e));
    };
    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "cpucaps")]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(short, long, default_value = "kubevirt")]
    kubevirt_ns: String,

    #[clap(short, long, default_value = "kubevirt.io=virt-handler")]
    selector: String,

    #[clap(short, long, default_value = "quay.io/kubevirt/virt-launcher:v1.7.1")]
    debugger_image: String,

    #[clap(short = 't', long, default_value = "3600")]
    debugger_ttl_seconds: u64,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
