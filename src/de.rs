use quick_xml::de;
use serde::de::DeserializeOwned;
use std::error::Error;
use std::io::BufRead;

pub mod types;

pub fn from_reader<R, T>(r: R) -> Result<T, Box<dyn Error>>
where
    R: BufRead,
    T: DeserializeOwned,
{
    let obj = de::from_reader(r)?;
    Ok(obj)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;
    use types::DomainCapabilities;

    #[test]
    fn test_from_reader_domcapabilities() {
        let filename = "testdata/virsh_domcapabilities.xml";
        let path = Path::new(filename);
        let xml_file = File::open(path).unwrap();
        let buf = BufReader::new(xml_file);
        let obj: DomainCapabilities = from_reader(buf).unwrap();

        assert_eq!(obj.path, "/usr/bin/qemu-system-x86_64");
        assert_eq!(obj.domain, "kvm");
        assert_eq!(obj.machine, "pc-q35-9.2");
        assert_eq!(obj.arch, "x86_64");
        assert_eq!(obj.vcpu.max, "4096");
        assert_eq!(obj.iothreads.supported, "yes");
        assert_eq!(obj.os.supported, "yes");

        let mode_host_passthrough = obj
            .cpu
            .mode
            .iter()
            .find(|m| m.name == "host-passthrough")
            .unwrap();
        assert_eq!(mode_host_passthrough.supported, "yes");

        let host_model = obj
            .cpu
            .mode
            .iter()
            .find(|m| m.name == "host-model")
            .unwrap();
        assert_eq!(host_model.supported, "yes");
        if let Some(v) = &host_model.model {
            if let Some(model) = v.first() {
                assert_eq!(model.text.as_ref().unwrap(), "EPYC-Genoa");
            } else {
                panic!("model of the 'host_model' mode is empty, expected it to be 'EPYC-Genoa'");
            }
            assert_eq!(v.len(), 1);
        } else {
            panic!("expected model of the 'host_model' mode to be Some, but got None");
        }
        assert_eq!(host_model.vendor.as_ref().unwrap(), "AMD");

        assert_eq!(host_model.feature.as_ref().unwrap().len(), 29);
        let disable_features = ["pcid", "la57", "vnmi", "auto-ibrs"];
        if let Some(features) = &host_model.feature {
            features
                .iter()
                .filter(|f| f.policy == "disable")
                .for_each(|f| {
                    assert!(
                        disable_features.contains(&f.name.as_str()),
                        "unexpected disabled feature '{}'",
                        f.name
                    );
                });
        } else {
            panic!(
                "expected 'disabled' features of the 'host_model' mode to be Some, but got None"
            );
        }

        let require_features = [
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
        if let Some(features) = &host_model.feature {
            features
                .iter()
                .filter(|f| f.policy == "require")
                .for_each(|f| {
                    assert!(
                        require_features.contains(&f.name.as_str()),
                        "unexpected required feature '{}'",
                        f.name
                    );
                });
        } else {
            panic!(
                "expected 'required' features of the 'host_model' mode to be Some, but got None"
            );
        }
    }
}
