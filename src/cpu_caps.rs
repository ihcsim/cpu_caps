use crate::de::types::{capabilities, supported_features, virsh_domcapabilities};

pub struct CpuCaps {
    global_caps: Vec<NodeCaps>,
    nodes_caps: Vec<NodeCaps>,
}

struct LibvirtData<'a> {
    caps: &'a capabilities::Capabilities,
    domcaps: &'a virsh_domcapabilities::DomainCapabilities,
    cpu: &'a supported_features::Cpu,
    virsh_version: String,
    virt_launcher_version: String,
}

impl CpuCaps {
    pub fn new(
        node_names: Vec<String>,
        caps: &capabilities::Capabilities,
        domcaps: &virsh_domcapabilities::DomainCapabilities,
        cpu: &supported_features::Cpu,
        virsh_version: &str,
        virt_launcher_version: &str,
    ) -> CpuCaps {
        let global_caps = Vec::new();
        let mut nodes_caps = Vec::new();
        let libvirt_data = &LibvirtData {
            caps,
            domcaps,
            cpu,
            virsh_version: virsh_version.to_string(),
            virt_launcher_version: virt_launcher_version.to_string(),
        };

        for node_name in node_names {
            nodes_caps.push(NodeCaps::new(node_name, libvirt_data));
        }
        CpuCaps {
            global_caps,
            nodes_caps,
        }
    }
}

struct NodeCaps {
    node_name: String,
    host_cpu_model: HostCpuModel,
    supported_features: Vec<String>,
    supported_models: Vec<String>,
    virsh_version: String,
    virt_launcher_version: String,
}

impl NodeCaps {
    fn new(node_name: String, libvirt_data: &LibvirtData) -> NodeCaps {
        NodeCaps {
            node_name,
            host_cpu_model: HostCpuModel {
                name: String::new(),
                vendor: String::new(),
                required_features: Vec::new(),
            },
            supported_features: NodeCaps::supported_features(libvirt_data.cpu),
            supported_models: NodeCaps::supported_models(libvirt_data.domcaps),
            virsh_version: libvirt_data.virsh_version.clone(),
            virt_launcher_version: libvirt_data.virt_launcher_version.clone(),
        }
    }

    fn supported_features(cpu: &supported_features::Cpu) -> Vec<String> {
        let mut supported_features = Vec::new();
        for feature in &cpu.feature {
            if feature.policy == "require" {
                supported_features.push(feature.name.clone());
            }
        }
        supported_features
    }

    fn supported_models(domcaps: &virsh_domcapabilities::DomainCapabilities) -> Vec<String> {
        let mut supported_models = Vec::new();
        for mode in &domcaps.cpu.mode {
            if let Some(models) = &mode.model {
                for model in models {
                    if let Some(model_name) = &model.text
                        && let Some(usable) = &model.usable
                        && usable == "yes"
                        && !is_obsolete_model(model_name)
                    {
                        supported_models.push(model_name.clone());
                    }
                }
            }
        }
        supported_models
    }
}

struct HostCpuModel {
    name: String,
    vendor: String,
    required_features: Vec<String>,
}

static OBSOLETE_CPU_MODELS: [&str; 34] = [
    "486",
    "486-v1",
    "pentium",
    "pentium-v1",
    "pentium2",
    "pentium2-v1",
    "pentium3",
    "pentium3-v1",
    "pentiumpro",
    "pentiumpro-v1",
    "coreduo",
    "coreduo-v1",
    "n270",
    "n270-v1",
    "core2duo",
    "core2duo-v1",
    "Conroe",
    "Conroe-v1",
    "athlon",
    "athlon-v1",
    "phenom",
    "phenom-v1",
    "qemu64",
    "qemu64-v1",
    "qemu32",
    "qemu32-v1",
    "kvm64",
    "kvm64-v1",
    "kvm32",
    "kvm32-v1",
    "Opteron_G1",
    "Opteron_G1-v1",
    "Opteron_G2",
    "Opteron_G2-v1",
];

fn is_obsolete_model(cpu_model: &str) -> bool {
    OBSOLETE_CPU_MODELS.contains(&cpu_model)
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::io::BufReader;
    use std::path::Path;

    use super::NodeCaps;
    use crate::de;
    use crate::de::types::{supported_features, virsh_domcapabilities};

    #[test]
    fn test_supported_models() {
        let path = Path::new("testdata").join("virsh_domcapabilities.xml");
        let raw = fs::read_to_string(path).unwrap();
        let reader = BufReader::new(raw.as_bytes());
        let domcaps: virsh_domcapabilities::DomainCapabilities = de::from_reader(reader).unwrap();
        let expected = vec![
            "Denverton-v2",
            "Denverton-v3",
            "Dhyana",
            "Dhyana-v1",
            "Dhyana-v2",
            "EPYC",
            "EPYC-IBPB",
            "EPYC-Rome",
            "EPYC-Rome-v1",
            "EPYC-Rome-v2",
            "EPYC-Rome-v3",
            "EPYC-Rome-v4",
            "EPYC-v1",
            "EPYC-v2",
            "EPYC-v3",
            "EPYC-v4",
            "IvyBridge",
            "IvyBridge-IBRS",
            "IvyBridge-v1",
            "IvyBridge-v2",
            "Nehalem",
            "Nehalem-IBRS",
            "Nehalem-v1",
            "Nehalem-v2",
            "Opteron_G3",
            "Opteron_G3-v1",
            "Penryn",
            "Penryn-v1",
            "SandyBridge",
            "SandyBridge-IBRS",
            "SandyBridge-v1",
            "SandyBridge-v2",
            "Westmere",
            "Westmere-IBRS",
            "Westmere-v1",
            "Westmere-v2",
        ];
        let actual = NodeCaps::supported_models(&domcaps);
        assert_eq!(actual.len(), 36);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_supported_features() {
        let path = Path::new("testdata").join("supported_features.xml");
        let raw = fs::read_to_string(path).unwrap();
        let reader = BufReader::new(raw.as_bytes());
        let cpus: supported_features::Cpu = de::from_reader(reader).unwrap();
        let expected = vec![
            "3dnowprefetch",
            "abm",
            "adx",
            "aes",
            "amd-psfd",
            "amd-ssbd",
            "amd-stibp",
            "apic",
            "arat",
            "arch-capabilities",
            "avx",
            "avx2",
            "avx512-bf16",
            "avx512-vpopcntdq",
            "avx512bitalg",
            "avx512bw",
            "avx512cd",
            "avx512dq",
            "avx512f",
            "avx512ifma",
            "avx512vbmi",
            "avx512vbmi2",
            "avx512vl",
            "avx512vnni",
            "bmi1",
            "bmi2",
            "clflush",
            "clflushopt",
            "clwb",
            "clzero",
            "cmov",
            "cmp_legacy",
            "cr8legacy",
            "cx16",
            "cx8",
            "de",
            "erms",
            "f16c",
            "flushbyasid",
            "fma",
            "fpu",
            "fsgsbase",
            "fsrm",
            "fxsr",
            "fxsr_opt",
            "gds-no",
            "gfni",
            "hypervisor",
            "ibpb",
            "ibrs",
            "invpcid",
            "lahf_lm",
            "lbrv",
            "lfence-always-serializing",
            "lm",
            "mca",
            "mce",
            "mds-no",
            "misalignsse",
            "mmx",
            "mmxext",
            "movbe",
            "msr",
            "mtrr",
            "no-nested-data-bp",
            "npt",
            "nrip-save",
            "null-sel-clr-base",
            "nx",
            "osvw",
            "overflow-recov",
            "pae",
            "pat",
            "pause-filter",
            "pclmuldq",
            "pdpe1gb",
            "perfctr_core",
            "pfthreshold",
            "pge",
            "pku",
            "pni",
            "popcnt",
            "pschange-mc-no",
            "pse",
            "pse36",
            "rdctl-no",
            "rdpid",
            "rdrand",
            "rdseed",
            "rdtscp",
            "rfds-no",
            "sep",
            "sha-ni",
            "skip-l1dfl-vmentry",
            "smap",
            "smep",
            "spec-ctrl",
            "ssbd",
            "sse",
            "sse2",
            "sse4.1",
            "sse4.2",
            "sse4a",
            "ssse3",
            "stibp",
            "stibp-always-on",
            "succor",
            "svm",
            "svme-addr-chk",
            "syscall",
            "tsc",
            "tsc-deadline",
            "tsc-scale",
            "tsc_adjust",
            "umip",
            "vaes",
            "vgif",
            "virt-ssbd",
            "vmcb-clean",
            "vme",
            "vpclmulqdq",
            "wbnoinvd",
            "x2apic",
            "xgetbv1",
            "xsave",
            "xsavec",
            "xsaveerptr",
            "xsaveopt",
            "xsaves",
        ];
        let actual = NodeCaps::supported_features(&cpus);
        assert_eq!(actual.len(), 129);
        assert_eq!(expected, actual);
    }
}
