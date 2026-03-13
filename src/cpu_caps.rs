use crate::de::types::{
    capabilities, supported_features, virsh_domcapabilities::DomainCapabilities,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn compute(
    node_names: Vec<&str>,
    caps: &capabilities::Capabilities,
    domcaps: &DomainCapabilities,
    cpu: &supported_features::Cpu,
    virsh_version: &str,
    virt_launcher_version: &str,
) -> CpuCaps {
    let libvirt_data = &LibvirtData {
        caps,
        domcaps,
        cpu,
        virsh_version: virsh_version.to_string(),
        virt_launcher_version: virt_launcher_version.to_string(),
    };

    let mut model_to_nodes = HashMap::new();
    let mut nodes_caps = Vec::new();
    for node_name in &node_names {
        let node_caps = NodeCaps::new(node_name.to_string(), libvirt_data);
        nodes_caps.push(node_caps.clone());

        for model in node_caps.supported_models {
            model_to_nodes
                .entry(model)
                .or_insert(Vec::new())
                .push(node_caps.node_name.clone());
        }
    }

    let mut global_caps = Vec::new();
    for (model, nodes) in model_to_nodes {
        if nodes.len() == node_names.len() {
            global_caps.push(model);
        }
    }

    CpuCaps {
        global_caps,
        nodes_caps,
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CpuCaps {
    global_caps: Vec<String>,
    nodes_caps: Vec<NodeCaps>,
}

impl CpuCaps {
    pub fn to_yaml(&self) -> Result<String, serde_yaml_ng::Error> {
        serde_yaml_ng::to_string(self)
    }
}

struct LibvirtData<'a> {
    caps: &'a capabilities::Capabilities,
    domcaps: &'a DomainCapabilities,
    cpu: &'a supported_features::Cpu,
    virsh_version: String,
    virt_launcher_version: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
            host_cpu_model: HostCpuModel::new(libvirt_data.domcaps),
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

    fn supported_models(domcaps: &DomainCapabilities) -> Vec<String> {
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct HostCpuModel {
    name: String,
    vendor: String,
    required_features: Vec<String>,
}

impl HostCpuModel {
    fn new(domcaps: &DomainCapabilities) -> HostCpuModel {
        let mode = domcaps
            .cpu
            .mode
            .iter()
            .find(|mode| mode.name == "host-model");

        let (name, vendor) = match mode {
            Some(mode) => {
                let mut model = "".to_string();
                if let Some(models) = &mode.model
                    && !models.is_empty()
                    && let Some(v) = &models[0].text
                {
                    model = v.clone()
                };

                let vendor = match &mode.vendor {
                    Some(v) => v.clone(),
                    None => "".to_string(),
                };

                (model, vendor)
            }
            None => ("".to_string(), "".to_string()),
        };

        HostCpuModel {
            name,
            vendor,
            required_features: HostCpuModel::required_features(domcaps),
        }
    }

    fn required_features(domcaps: &DomainCapabilities) -> Vec<String> {
        let mode = domcaps
            .cpu
            .mode
            .iter()
            .find(|model| model.name == "host-model");
        if let Some(mode) = mode
            && let Some(features) = &mode.feature
        {
            return features
                .iter()
                .filter(|feature| feature.policy == "require")
                .map(|feature| feature.name.clone())
                .collect();
        }
        Vec::new()
    }
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

    use super::*;
    use crate::de;
    use crate::de::types::{supported_features::Cpu, virsh_domcapabilities::DomainCapabilities};
    use std::fs::File;

    #[test]
    fn test_compute_one_node() {
        let path = Path::new("testdata").join("virsh_domcapabilities.xml");
        let xml_file = File::open(path).unwrap();
        let buf = BufReader::new(xml_file);
        let domcaps: DomainCapabilities = de::from_reader(buf).unwrap();

        let path = Path::new("testdata").join("capabilities.xml");
        let xml_file = File::open(path).unwrap();
        let buf = BufReader::new(xml_file);
        let caps: capabilities::Capabilities = de::from_reader(buf).unwrap();

        let path = Path::new("testdata").join("supported_features.xml");
        let xml_file = File::open(path).unwrap();
        let buf = BufReader::new(xml_file);
        let cpu: Cpu = de::from_reader(buf).unwrap();

        let node_names = vec!["isim-dev"];
        let virsh_version = r#"Compiled against library: libvirt 11.0.0
Using library: libvirt 11.0.0
Using API: QEMU 11.0.0
"#;
        let virt_launcher_version = "1.6.3";
        let mut cpu_caps = compute(
            node_names,
            &caps,
            &domcaps,
            &cpu,
            virsh_version,
            virt_launcher_version,
        );

        let mut expected_supported_models = vec![
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
        let mut expected_supported_features = vec![
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
        let expected_host_cpu_model = HostCpuModel::new(&domcaps);

        expected_supported_features.sort();
        expected_supported_models.sort();

        let node_caps = cpu_caps.nodes_caps.last().unwrap();
        assert_eq!(node_caps.node_name, "isim-dev");
        assert_eq!(node_caps.host_cpu_model, expected_host_cpu_model);
        assert_eq!(node_caps.supported_features, expected_supported_features);
        assert_eq!(node_caps.supported_models, expected_supported_models);
        assert_eq!(node_caps.virsh_version, virsh_version);
        assert_eq!(node_caps.virt_launcher_version, virt_launcher_version);

        cpu_caps.global_caps.sort();
        assert_eq!(cpu_caps.global_caps, expected_supported_models);
    }

    #[test]
    fn test_supported_models() {
        let path = Path::new("testdata").join("virsh_domcapabilities.xml");
        let raw = fs::read_to_string(path).unwrap();
        let reader = BufReader::new(raw.as_bytes());
        let domcaps: DomainCapabilities = de::from_reader(reader).unwrap();
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
        let cpus: Cpu = de::from_reader(reader).unwrap();
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

    #[test]
    fn test_host_model_name_and_vendor() {
        let path = Path::new("testdata").join("virsh_domcapabilities.xml");
        let raw = fs::read_to_string(path).unwrap();
        let reader = BufReader::new(raw.as_bytes());
        let domcaps: DomainCapabilities = de::from_reader(reader).unwrap();
        let host_cpu_model = HostCpuModel::new(&domcaps);
        assert_eq!(host_cpu_model.name, "EPYC-Genoa");
        assert_eq!(host_cpu_model.vendor, "AMD");
    }

    #[test]
    fn test_host_model_required_features() {
        let path = Path::new("testdata").join("virsh_domcapabilities.xml");
        let raw = fs::read_to_string(path).unwrap();
        let reader = BufReader::new(raw.as_bytes());
        let domcaps: DomainCapabilities = de::from_reader(reader).unwrap();
        let expected = vec![
            "x2apic",
            "tsc-deadline",
            "hypervisor",
            "tsc_adjust",
            "spec-ctrl",
            "stibp",
            "arch-capabilities",
            "ssbd",
            "cmp_legacy",
            "overflow-recov",
            "succor",
            "virt-ssbd",
            "lbrv",
            "tsc-scale",
            "vmcb-clean",
            "flushbyasid",
            "pause-filter",
            "pfthreshold",
            "vgif",
            "rdctl-no",
            "skip-l1dfl-vmentry",
            "mds-no",
            "pschange-mc-no",
            "gds-no",
            "rfds-no",
        ];
        let actual = HostCpuModel::required_features(&domcaps);
        assert_eq!(actual.len(), 25);
        assert_eq!(expected, actual);
    }
}
