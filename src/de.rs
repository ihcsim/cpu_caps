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
    use types::capabilities::Capabilities;
    use types::supported_features::Cpu;
    use types::virsh_domcapabilities::DomainCapabilities;

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

    #[test]
    fn test_from_reader_capabilities() {
        let filename = "testdata/capabilities.xml";
        let path = Path::new(filename);
        let xml_file = File::open(path).unwrap();
        let buf = BufReader::new(xml_file);
        let obj: Capabilities = from_reader(buf).unwrap();

        assert_eq!(obj.host.uuid, "c8710b37-a445-4fc3-bb74-1917f4ef973e");
        assert_eq!(obj.host.cpu.arch, "x86_64");
        assert_eq!(obj.host.cpu.model, "EPYC-Rome-v3");
        assert_eq!(obj.host.cpu.vendor, "AMD");
        assert_eq!(obj.host.cpu.topology.sockets, "8");
        assert_eq!(obj.host.cpu.topology.dies, "1");
        assert_eq!(obj.host.cpu.topology.clusters, "1");
        assert_eq!(obj.host.cpu.topology.cores, "1");
        assert_eq!(obj.host.cpu.topology.threads, "1");
        assert_eq!(obj.host.cpu.feature.len(), 50);
        assert_eq!(obj.host.cpu.pages.len(), 3);

        assert_eq!(obj.host.iommu.support, "no");
        assert_eq!(obj.host.topology.cells.num, "1");
        assert_eq!(obj.host.topology.cells.cell.id, "0");
        assert_eq!(obj.host.topology.cells.cell.pages.len(), 3);
        assert_eq!(obj.host.topology.cells.cell.cpus.num, "8");
        assert_eq!(obj.host.topology.cells.cell.cpus.cpu.len(), 8);

        assert_eq!(obj.guest.len(), 2);

        if let Some(guest) = obj.guest.last() {
            assert_eq!(guest.os_type, "hvm");
            assert_eq!(guest.arch.name, "x86_64");
            assert_eq!(guest.arch.wordsize, "64");
            assert_eq!(guest.arch.emulator, "/usr/bin/qemu-system-x86_64");
            assert_eq!(guest.arch.domain.first().unwrap().domain_type, "qemu");
            assert_eq!(guest.arch.domain.last().unwrap().domain_type, "kvm");

            let deprecated_machines = [
                "pc-q35-5.2",
                "pc-i440fx-2.12",
                "pc-i440fx-6.2",
                "pc-q35-4.2",
                "pc-i440fx-2.5",
                "pc-i440fx-4.2",
                "pc-i440fx-5.2",
                "pc-q35-2.7",
                "pc-i440fx-2.7",
                "pc-q35-6.1",
                "pc-q35-2.4",
                "pc-q35-2.10",
                "pc-q35-5.1",
                "pc-q35-2.9",
                "pc-i440fx-2.11",
                "pc-q35-3.1",
                "pc-i440fx-6.1",
                "pc-q35-4.1",
                "pc-i440fx-2.4",
                "pc-i440fx-4.1",
                "pc-i440fx-5.1",
                "pc-i440fx-2.9",
                "pc-q35-2.6",
                "pc-i440fx-3.1",
                "pc-q35-2.12",
                "pc-q35-6.0",
                "pc-i440fx-2.6",
                "pc-q35-4.0.1",
                "pc-q35-5.0",
                "pc-q35-2.8",
                "pc-i440fx-2.10",
                "pc-q35-3.0",
                "pc-i440fx-6.0",
                "pc-q35-4.0",
                "pc-i440fx-4.0",
                "pc-i440fx-5.0",
                "pc-i440fx-2.8",
                "pc-q35-6.2",
                "pc-q35-2.5",
                "pc-i440fx-3.0",
                "pc-q35-2.11",
            ];

            let machines = [
                "pc-i440fx-9.2",
                "pc",
                "xenpv",
                "pc-q35-9.1",
                "pc-q35-7.1",
                "pc-q35-8.1",
                "pc-i440fx-8.1",
                "xenfv-3.1",
                "xenfv",
                "pc-i440fx-9.1",
                "pc-i440fx-7.1",
                "isapc",
                "pc-q35-9.0",
                "pc-q35-7.0",
                "pc-q35-8.0",
                "pc-i440fx-8.0",
                "xenpvh",
                "pc-i440fx-9.0",
                "nitro-enclave",
                "pc-i440fx-7.0",
                "pc-q35-9.2",
                "q35",
                "pc-q35-7.2",
                "xenfv-4.2",
                "microvm",
                "pc-q35-8.2",
                "pc-i440fx-8.2",
                "pc-i440fx-7.2",
            ];
            guest.arch.machine.iter().for_each(|m| {
                if let Some(deprecated) = m.deprecated.as_ref()
                    && deprecated == "yes"
                {
                    assert!(
                        deprecated_machines.contains(&m.text.as_ref().unwrap().as_str()),
                        "unexpected deprecated machine '{}'",
                        &m.text.as_ref().unwrap()
                    );
                } else {
                    assert!(
                        machines.contains(&m.text.as_ref().unwrap().as_str()),
                        "unexpected non-deprecated machine '{}'",
                        &m.text.as_ref().unwrap()
                    );
                }
            });
        } else {
            panic!("expected at least one guest, but got none");
        }
    }

    #[test]
    fn test_from_reader_supported_features() {
        let filename = "testdata/supported_features.xml";
        let path = Path::new(filename);
        let xml_file = File::open(path).unwrap();
        let buf = BufReader::new(xml_file);
        let obj: Cpu = from_reader(buf).unwrap();

        assert_eq!(obj.model.text.unwrap(), "EPYC-Genoa");
        assert_eq!(obj.vendor, "AMD");
        assert_eq!(obj.maxphysaddr.mode, "passthrough");
        assert_eq!(obj.maxphysaddr.limit, "48");

        let required_features = [
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

        let disabled_features = ["auto-ibrs", "la57", "pcid", "vnmi"];

        obj.feature.iter().for_each(|f| {
            if f.policy == "require" {
                assert!(
                    required_features.contains(&f.name.as_str()),
                    "unexpected required feature '{}'",
                    f.name
                );
            } else if f.policy == "disable" {
                assert!(
                    disabled_features.contains(&f.name.as_str()),
                    "unexpected disabled feature '{}'",
                    f.name
                );
            } else {
                panic!(
                    "unexpected policy '{}', expected 'require' or 'disable'",
                    f.policy
                );
            }
        });
    }
}
