use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct DomainCapabilities {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub path: String,
    pub domain: String,
    pub machine: String,
    pub arch: String,
    pub vcpu: Vcpu,
    pub iothreads: Iothreads,
    pub os: Os,
    pub cpu: Cpu,
    #[serde(rename = "memoryBacking")]
    pub memory_backing: MemoryBacking,
    pub devices: Devices,
    pub features: Features,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Vcpu {
    #[serde(rename = "@max")]
    pub max: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Iothreads {
    #[serde(rename = "@supported")]
    pub supported: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Os {
    #[serde(rename = "@supported")]
    pub supported: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "enum")]
    pub os_enum: OsEnum,
    pub loader: Loader,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct OsEnum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Loader {
    #[serde(rename = "@supported")]
    pub supported: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub value: Vec<String>,
    #[serde(rename = "enum")]
    pub loader_enum: Vec<LoaderEnum>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct LoaderEnum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Cpu {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub mode: Vec<Mode>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Mode {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@supported")]
    pub supported: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "enum")]
    pub mode_enum: Option<ModeEnum>,
    pub model: Option<Vec<Model>>,
    pub vendor: Option<String>,
    pub maxphysaddr: Option<Maxphysaddr>,
    pub feature: Option<Vec<ModeFeature>>,
    pub blockers: Option<Vec<Blockers>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct ModeEnum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Model {
    #[serde(rename = "@fallback")]
    pub fallback: Option<String>,
    #[serde(rename = "@canonical")]
    pub canonical: Option<String>,
    #[serde(rename = "@vendor")]
    pub vendor: Option<String>,
    #[serde(rename = "@usable")]
    pub usable: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Maxphysaddr {
    #[serde(rename = "@mode")]
    pub mode: String,
    #[serde(rename = "@limit")]
    pub limit: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct ModeFeature {
    #[serde(rename = "@policy")]
    pub policy: String,
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Blockers {
    #[serde(rename = "@model")]
    pub model: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub feature: Vec<BlockersFeature>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct BlockersFeature {
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct MemoryBacking {
    #[serde(rename = "@supported")]
    pub supported: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "enum")]
    pub memory_backing_enum: MemoryBackingEnum,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct MemoryBackingEnum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Devices {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub disk: Disk,
    pub graphics: Graphics,
    pub video: Video,
    pub hostdev: Hostdev,
    pub rng: Rng,
    pub filesystem: Filesystem,
    pub tpm: Tpm,
    pub redirdev: Redirdev,
    pub channel: Channel,
    pub crypto: Crypto,
    pub interface: Interface,
    pub panic: Panic,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Disk {
    #[serde(rename = "@supported")]
    pub supported: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "enum")]
    pub disk_enum: Vec<DiskEnum>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct DiskEnum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Graphics {
    #[serde(rename = "@supported")]
    pub supported: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "enum")]
    pub graphics_enum: GraphicsEnum,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct GraphicsEnum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Video {
    #[serde(rename = "@supported")]
    pub supported: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "enum")]
    pub video_enum: VideoEnum,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct VideoEnum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Hostdev {
    #[serde(rename = "@supported")]
    pub supported: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "enum")]
    pub hostdev_enum: Vec<HostdevEnum>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct HostdevEnum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub value: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Rng {
    #[serde(rename = "@supported")]
    pub supported: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "enum")]
    pub rng_enum: Vec<RngEnum>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct RngEnum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Filesystem {
    #[serde(rename = "@supported")]
    pub supported: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "enum")]
    pub filesystem_enum: FilesystemEnum,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct FilesystemEnum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Tpm {
    #[serde(rename = "@supported")]
    pub supported: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "enum")]
    pub tpm_enum: Vec<TpmEnum>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct TpmEnum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Redirdev {
    #[serde(rename = "@supported")]
    pub supported: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "enum")]
    pub redirdev_enum: RedirdevEnum,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct RedirdevEnum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Channel {
    #[serde(rename = "@supported")]
    pub supported: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "enum")]
    pub channel_enum: ChannelEnum,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct ChannelEnum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Crypto {
    #[serde(rename = "@supported")]
    pub supported: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "enum")]
    pub crypto_enum: Vec<CryptoEnum>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct CryptoEnum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub value: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Interface {
    #[serde(rename = "@supported")]
    pub supported: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "enum")]
    pub interface_enum: InterfaceEnum,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct InterfaceEnum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Panic {
    #[serde(rename = "@supported")]
    pub supported: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "enum")]
    pub panic_enum: PanicEnum,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct PanicEnum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Features {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub gic: Gic,
    pub vmcoreinfo: Vmcoreinfo,
    pub genid: Genid,
    #[serde(rename = "backingStoreInput")]
    pub backing_store_input: BackingStoreInput,
    pub backup: Backup,
    #[serde(rename = "async-teardown")]
    pub async_teardown: AsyncTeardown,
    pub ps2: Ps2,
    pub sev: Sev,
    pub sgx: Sgx,
    pub hyperv: Hyperv,
    #[serde(rename = "launchSecurity")]
    pub launch_security: LaunchSecurity,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Gic {
    #[serde(rename = "@supported")]
    pub supported: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Vmcoreinfo {
    #[serde(rename = "@supported")]
    pub supported: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Genid {
    #[serde(rename = "@supported")]
    pub supported: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct BackingStoreInput {
    #[serde(rename = "@supported")]
    pub supported: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Backup {
    #[serde(rename = "@supported")]
    pub supported: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct AsyncTeardown {
    #[serde(rename = "@supported")]
    pub supported: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Ps2 {
    #[serde(rename = "@supported")]
    pub supported: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Sev {
    #[serde(rename = "@supported")]
    pub supported: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Sgx {
    #[serde(rename = "@supported")]
    pub supported: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Hyperv {
    #[serde(rename = "@supported")]
    pub supported: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "enum")]
    pub hyperv_enum: HypervEnum,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct HypervEnum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct LaunchSecurity {
    #[serde(rename = "@supported")]
    pub supported: String,
}
