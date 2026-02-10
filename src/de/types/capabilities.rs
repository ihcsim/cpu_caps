use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Capabilities {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub host: Host,
    pub guest: Vec<Guest>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Host {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub uuid: String,
    pub cpu: HostCpu,
    pub power_management: PowerManagement,
    pub iommu: Iommu,
    pub migration_features: MigrationFeatures,
    pub topology: HostTopology,
    pub cache: Cache,
    pub secmodel: Vec<Secmodel>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HostCpu {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub arch: String,
    pub model: String,
    pub vendor: String,
    pub microcode: Microcode,
    pub signature: Signature,
    pub topology: CpuTopology,
    pub maxphysaddr: Maxphysaddr,
    pub feature: Vec<Feature>,
    pub pages: Vec<CpuPages>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Microcode {
    #[serde(rename = "@version")]
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Signature {
    #[serde(rename = "@family")]
    pub family: String,
    #[serde(rename = "@model")]
    pub model: String,
    #[serde(rename = "@stepping")]
    pub stepping: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CpuTopology {
    #[serde(rename = "@sockets")]
    pub sockets: String,
    #[serde(rename = "@dies")]
    pub dies: String,
    #[serde(rename = "@clusters")]
    pub clusters: String,
    #[serde(rename = "@cores")]
    pub cores: String,
    #[serde(rename = "@threads")]
    pub threads: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Maxphysaddr {
    #[serde(rename = "@mode")]
    pub mode: String,
    #[serde(rename = "@bits")]
    pub bits: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Feature {
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CpuPages {
    #[serde(rename = "@unit")]
    pub unit: String,
    #[serde(rename = "@size")]
    pub size: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PowerManagement {
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Iommu {
    #[serde(rename = "@support")]
    pub support: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MigrationFeatures {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub live: Live,
    pub uri_transports: UriTransports,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Live {
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UriTransports {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub uri_transport: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HostTopology {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub cells: Cells,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Cells {
    #[serde(rename = "@num")]
    pub num: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub cell: Cell,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Cell {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub memory: Memory,
    pub pages: Vec<CellPages>,
    pub distances: Distances,
    pub cpus: Cpus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Memory {
    #[serde(rename = "@unit")]
    pub unit: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CellPages {
    #[serde(rename = "@unit")]
    pub unit: String,
    #[serde(rename = "@size")]
    pub size: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Distances {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub sibling: Sibling,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Sibling {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Cpus {
    #[serde(rename = "@num")]
    pub num: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub cpu: Vec<CpusCpu>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CpusCpu {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@socket_id")]
    pub socket_id: String,
    #[serde(rename = "@die_id")]
    pub die_id: String,
    #[serde(rename = "@cluster_id")]
    pub cluster_id: String,
    #[serde(rename = "@core_id")]
    pub core_id: String,
    #[serde(rename = "@siblings")]
    pub siblings: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Cache {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub bank: Vec<Bank>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Bank {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@level")]
    pub level: String,
    #[serde(rename = "@type")]
    pub bank_type: String,
    #[serde(rename = "@size")]
    pub size: String,
    #[serde(rename = "@unit")]
    pub unit: String,
    #[serde(rename = "@cpus")]
    pub cpus: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Secmodel {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub model: String,
    pub doi: String,
    pub baselabel: Option<Vec<Baselabel>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Baselabel {
    #[serde(rename = "@type")]
    pub baselabel_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Guest {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub os_type: String,
    pub arch: GuestArch,
    pub features: Features,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GuestArch {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub wordsize: String,
    pub emulator: String,
    pub machine: Vec<Machine>,
    pub domain: Vec<Domain>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Machine {
    #[serde(rename = "@maxCpus")]
    pub max_cpus: String,
    #[serde(rename = "@canonical")]
    pub canonical: Option<String>,
    #[serde(rename = "@deprecated")]
    pub deprecated: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Domain {
    #[serde(rename = "@type")]
    pub domain_type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Features {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub pae: Option<Pae>,
    pub nonpae: Option<Nonpae>,
    pub acpi: Acpi,
    pub apic: Apic,
    pub cpuselection: Cpuselection,
    pub deviceboot: Deviceboot,
    pub disksnapshot: Disksnapshot,
    #[serde(rename = "externalSnapshot")]
    pub external_snapshot: ExternalSnapshot,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Pae {
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Nonpae {
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Acpi {
    #[serde(rename = "@default")]
    pub default: String,
    #[serde(rename = "@toggle")]
    pub toggle: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Apic {
    #[serde(rename = "@default")]
    pub default: String,
    #[serde(rename = "@toggle")]
    pub toggle: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Cpuselection {
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Deviceboot {
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Disksnapshot {
    #[serde(rename = "@default")]
    pub default: String,
    #[serde(rename = "@toggle")]
    pub toggle: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ExternalSnapshot {
}

