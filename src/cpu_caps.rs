use crate::de::types::{capabilities, supported_features, virsh_domcapabilities};

pub struct CpuCaps {
    global_caps: Vec<NodeCaps>,
    nodes_caps: Vec<NodeCaps>,
}

struct LibvirtData<'a> {
    caps: &'a capabilities::Capabilities,
    domcaps: &'a virsh_domcapabilities::DomainCapabilities,
    cpu: &'a supported_features::Cpu,
}

impl CpuCaps {
    pub fn new(
        node_names: Vec<String>,
        caps: &capabilities::Capabilities,
        domcaps: &virsh_domcapabilities::DomainCapabilities,
        cpu: &supported_features::Cpu,
    ) -> CpuCaps {
        let global_caps = Vec::new();
        let mut nodes_caps = Vec::new();
        let libvirt_data = &LibvirtData { caps, domcaps, cpu };

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
            supported_features: Vec::new(),
            supported_models: NodeCaps::supported_models(libvirt_data.domcaps),
            virsh_version: String::new(),
            virt_launcher_version: String::new(),
        }
    }

    fn supported_models(domcaps: &virsh_domcapabilities::DomainCapabilities) -> Vec<String> {
        let mut supported_models = Vec::new();
        for mode in &domcaps.cpu.mode {
            if let Some(models) = &mode.model {
                for model in models {
                    if let Some(model_name) = &model.text
                        && let Some(usable) = &model.usable
                        && usable == "yes"
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

#[cfg(test)]
mod test {
    use std::fs;
    use std::io::BufReader;
    use std::path::Path;

    use super::NodeCaps;
    use crate::de;
    use crate::de::types::virsh_domcapabilities;

    #[test]
    fn test_supported_models() {
        let path = Path::new("testdata").join("virsh_domcapabilities.xml");
        let raw = fs::read_to_string(path).unwrap();
        let reader = BufReader::new(raw.as_bytes());
        let domcaps: virsh_domcapabilities::DomainCapabilities = de::from_reader(reader).unwrap();
        let expected = vec![
            "486",
            "486-v1",
            "Conroe",
            "Conroe-v1",
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
            "Opteron_G1",
            "Opteron_G1-v1",
            "Opteron_G2",
            "Opteron_G2-v1",
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
            "kvm32",
            "kvm32-v1",
            "kvm64",
            "kvm64-v1",
            "pentium",
            "pentium-v1",
            "pentium2",
            "pentium2-v1",
            "pentium3",
            "pentium3-v1",
            "qemu32",
            "qemu32-v1",
            "qemu64",
            "qemu64-v1",
        ];
        let actual = NodeCaps::supported_models(&domcaps);
        assert_eq!(actual.len(), 58);
        assert_eq!(expected, actual);
    }
}
