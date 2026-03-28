use crate::de::types::{
    capabilities::Capabilities, supported_features::Cpu, virsh_domcapabilities::DomainCapabilities,
};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct LibvirtData {
    pub _caps: Capabilities,
    pub domcaps: DomainCapabilities,
    pub cpu: Cpu,
    pub node_name: String,
    pub virsh_version: String,
    pub virt_launcher_image: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeCaps {
    pub node_name: String,
    pub host_cpu_model: HostCpuModel,
    pub supported_features: Vec<String>,
    pub supported_models: Vec<String>,
    pub virsh_version: String,
    pub virt_launcher_image: String,
}

impl NodeCaps {
    pub fn new(data: &LibvirtData) -> NodeCaps {
        NodeCaps {
            node_name: data.node_name.clone(),
            host_cpu_model: HostCpuModel::new(&data.domcaps),
            supported_features: NodeCaps::supported_features(&data.cpu),
            supported_models: NodeCaps::supported_models(&data.domcaps),
            virsh_version: data.virsh_version.clone(),
            virt_launcher_image: data.virt_launcher_image.clone(),
        }
    }

    fn supported_features(cpu: &Cpu) -> Vec<String> {
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
                        && !super::is_obsolete_model(model_name)
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
pub struct HostCpuModel {
    name: String,
    vendor: String,
    required_features: Vec<String>,
}

impl HostCpuModel {
    pub fn new(domcaps: &DomainCapabilities) -> HostCpuModel {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::de::{self, types::supported_features::Cpu};
    use std::fs;
    use std::io::BufReader;
    use std::path::Path;

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
