# CPU Capabilities Discovery Tool

The `check_cpu_caps.sh` script discovers and collects the host CPU capabilities
information from all the KubeVirt `virt-handler` DaemonSet pods in the
`${KUBEVIRT_NAMESPACE}` namespace.

It allows us to compare the CPU capabilities information identified by different
versions of `virt-handler` and also identify any global CPU model candidates. A
CPU model is considered a globally supported candidate if more than half of the
nodes support it.

## Prerequisites

[KubeVirt](https://kubevirt.io/) must be running on the cluster.

Run the script in a Linux shell with the following tools in its `$PATH`:

* kubectl
* yq

The kubeconfig must be included in the shell `$PATH` with permissions to run
`kubectl [debug|cp|exec]` targeting the `${KUBEVIRT_NAMESPACE}` namespace.

## Usage

The examples in this usage section is performed on a
[Harvester](https://harvesterhci.io/) 1.5.2 cluster, with 2 nodes named
`isim-dev` and `isim-dev2`. It covers the scenario of running a new version of
`virt-launcher` on the 1.5.2 cluster.

To generate a report with information on the host CPU capabilities of all the
nodes on the cluster:

```sh
./check_cpu_caps.sh
```

```sh
⚙️ discovering host and domain virt capabilities...
➡️ version: registry.suse.com/suse/sles/15.6/virt-launcher:1.4.1-150600.5.21.2
  ⏳ checking pod harvester-system/virt-handler-58bwp...
     running debugger virt-handler-58bwp/debug-20251205-102708...
tar: Removing leading `/' from member names
  ⏳ checking pod harvester-system/virt-handler-t2qlz...
     running debugger virt-handler-t2qlz/debug-20251205-102713...
tar: Removing leading `/' from member names
⚙️ generating report summary...
  📝 output saved to ./out-20251205-102708.tar.gz
```

The report is saved to `./out-20251205-102708.tar.gz`.

Untar the `.tar.gz` file:

```sh
tar xvf ./out-20251205-102708.tar.gz
```

The report contains the CPU capabilities information collected from all the
nodes in the cluster:

```sh
./out-20251205-102708/
./out-20251205-102708/report.yaml
./out-20251205-102708/isim-dev2/
./out-20251205-102708/isim-dev2/1.4.1-150600.5.21.2/
./out-20251205-102708/isim-dev2/1.4.1-150600.5.21.2/supported_features.xml
./out-20251205-102708/isim-dev2/1.4.1-150600.5.21.2/virsh_domcapabilities.xml
./out-20251205-102708/isim-dev2/1.4.1-150600.5.21.2/.done
./out-20251205-102708/isim-dev2/1.4.1-150600.5.21.2/capabilities.xml
./out-20251205-102708/isim-dev2/1.4.1-150600.5.21.2/.version
./out-20251205-102708/isim-dev/
./out-20251205-102708/isim-dev/1.4.1-150600.5.21.2/
./out-20251205-102708/isim-dev/1.4.1-150600.5.21.2/supported_features.xml
./out-20251205-102708/isim-dev/1.4.1-150600.5.21.2/virsh_domcapabilities.xml
./out-20251205-102708/isim-dev/1.4.1-150600.5.21.2/.done
./out-20251205-102708/isim-dev/1.4.1-150600.5.21.2/capabilities.xml
./out-20251205-102708/isim-dev/1.4.1-150600.5.21.2/.version
```

Use `yq` to parse the `report.yaml` file:

```sh
yq . ./out-20251205-102708/report.yaml
```

<details>
<summary>Expand to see full report output.</summary>

```yaml
global:
  supported_cpu_models:
  - EPYC-Rome-v3: isim-dev2
  - EPYC-Rome-v2: isim-dev2
  - EPYC-Rome-v1: isim-dev2
  - EPYC-Rome-v4: isim-dev2
  - Penryn: isim-dev2
  - EPYC-v4: isim-dev2
  - EPYC-v1: isim-dev2
  - EPYC-v3: isim-dev2
  - EPYC-v2: isim-dev2
  - Westmere-v1: isim-dev2
  - Westmere-v2: isim-dev2
  - IvyBridge-IBRS: isim-dev2
  - Denverton-v3: isim-dev2
  - Denverton-v2: isim-dev2
  - EPYC-Rome: isim-dev2
  - Westmere-IBRS: isim-dev2
  - Opteron_G3-v1: isim-dev2
  - Nehalem-IBRS: isim-dev2
  - Dhyana: isim-dev2
  - Nehalem: isim-dev2
  - SandyBridge: isim-dev2
  - IvyBridge: isim-dev2
  - IvyBridge-v1: isim-dev2
  - IvyBridge-v2: isim-dev2
  - Nehalem-v2: isim-dev2
  - Nehalem-v1: isim-dev2
  - SandyBridge-IBRS: isim-dev2
  - Westmere: isim-dev2
  - Penryn-v1: isim-dev2
  - Opteron_G3: isim-dev2
  - SandyBridge-v2: isim-dev2
  - SandyBridge-v1: isim-dev2
  - EPYC: isim-dev2
  - EPYC-IBPB: isim-dev2
  - Dhyana-v2: isim-dev2
  - Dhyana-v1: isim-dev2
nodes:
  isim-dev2:
  - virt_launcher: 1.6.3
    host_cpu_model:
      name: EPYC-Genoa
      vendor: AMD
      required_features:
      - x2apic
      - tsc-deadline
      - hypervisor
      - tsc_adjust
      - spec-ctrl
      - stibp
      - arch-capabilities
      - ssbd
      - cmp_legacy
      - overflow-recov
      - succor
      - virt-ssbd
      - lbrv
      - tsc-scale
      - vmcb-clean
      - flushbyasid
      - pause-filter
      - pfthreshold
      - vgif
      - rdctl-no
      - skip-l1dfl-vmentry
      - mds-no
      - pschange-mc-no
      - gds-no
      - rfds-no
    supported_models:
    - Denverton-v2
    - Denverton-v3
    - Dhyana
    - Dhyana-v1
    - Dhyana-v2
    - EPYC
    - EPYC-IBPB
    - EPYC-Rome
    - EPYC-Rome-v1
    - EPYC-Rome-v2
    - EPYC-Rome-v3
    - EPYC-Rome-v4
    - EPYC-v1
    - EPYC-v2
    - EPYC-v3
    - EPYC-v4
    - IvyBridge
    - IvyBridge-IBRS
    - IvyBridge-v1
    - IvyBridge-v2
    - Nehalem
    - Nehalem-IBRS
    - Nehalem-v1
    - Nehalem-v2
    - Opteron_G3
    - Opteron_G3-v1
    - Penryn
    - Penryn-v1
    - SandyBridge
    - SandyBridge-IBRS
    - SandyBridge-v1
    - SandyBridge-v2
    - Westmere
    - Westmere-IBRS
    - Westmere-v1
    supported_features:
    - 3dnowprefetch
    - abm
    - adx
    - aes
    - amd-psfd
    - amd-ssbd
    - amd-stibp
    - apic
    - arat
    - arch-capabilities
    - avx
    - avx2
    - avx512-bf16
    - avx512-vpopcntdq
    - avx512bitalg
    - avx512bw
    - avx512cd
    - avx512dq
    - avx512f
    - avx512ifma
    - avx512vbmi
    - avx512vbmi2
    - avx512vl
    - avx512vnni
    - bmi1
    - bmi2
    - clflush
    - clflushopt
    - clwb
    - clzero
    - cmov
    - cmp_legacy
    - cr8legacy
    - cx16
    - cx8
    - de
    - erms
    - f16c
    - flushbyasid
    - fma
    - fpu
    - fsgsbase
    - fsrm
    - fxsr
    - fxsr_opt
    - gds-no
    - gfni
    - hypervisor
    - ibpb
    - ibrs
    - invpcid
    - lahf_lm
    - lbrv
    - lfence-always-serializing
    - lm
    - mca
    - mce
    - mds-no
    - misalignsse
    - mmx
    - mmxext
    - movbe
    - msr
    - mtrr
    - no-nested-data-bp
    - npt
    - nrip-save
    - null-sel-clr-base
    - nx
    - osvw
    - overflow-recov
    - pae
    - pat
    - pause-filter
    - pclmuldq
    - pdpe1gb
    - perfctr_core
    - pfthreshold
    - pge
    - pku
    - pni
    - popcnt
    - pschange-mc-no
    - pse
    - pse36
    - rdctl-no
    - rdpid
    - rdrand
    - rdseed
    - rdtscp
    - rfds-no
    - sep
    - sha-ni
    - skip-l1dfl-vmentry
    - smap
    - smep
    - spec-ctrl
    - ssbd
    - sse
    - sse2
    - sse4.1
    - sse4.2
    - sse4a
    - ssse3
    - stibp
    - stibp-always-on
    - succor
    - svm
    - svme-addr-chk
    - syscall
    - tsc
    - tsc-deadline
    - tsc-scale
    - tsc_adjust
    - umip
    - vaes
    - vgif
    - virt-ssbd
    - vmcb-clean
    - vme
    - vpclmulqdq
    - wbnoinvd
    - x2apic
    - xgetbv1
    - xsave
    - xsavec
    - xsaveerptr
    - xsaveopt
    - xsaves
    virsh_version: |
      Compiled against library: libvirt 11.0.0
      Using library: libvirt 11.0.0
      Using API: QEMU 11.0.0
```

</details>

The following is a list of useful `yq` queries:

```sh
# show the host cpu model information on node isim-dev
yq .nodes.isim-dev.0.host_cpu_model out-20251205-102708/report.yaml

# show the supported cpu models on node isim-dev
yq .nodes.isim-dev.0.supported_models out-20251205-102708/report.yaml

# show the supported cpu features on node isim-dev2
yq .nodes.isim-dev2.0.supported_features out-20251205-102708/report.yaml

# show the libvirt and qemu versions on all nodes
yq '.nodes | to_entries | .[] | "node: \(.key)\n\(.value[0].virsh_version)"' out-20251205-102708/report.yaml
```

To check the CPU capabilities information generated by KubeVirt 1.6.3
`virt-launcher` (used in Harvester 1.6):

```sh
./check_cpu_caps.sh -i registry.opensuse.org/isv/rancher/harvester/containers/v1.7/15.7/suse/sles/15.7/virt-launcher:1.6.3
```

The new report includes the information generated by both the `virt-launcher:1.4.1` and `virt-launcher:1.6.3` containers:

```sh
⚙️ discovering host and domain virt capabilities...
➡️ version: registry.suse.com/suse/sles/15.6/virt-launcher:1.4.1-150600.5.21.2
  ⏳ checking pod harvester-system/virt-handler-58bwp...
     running debugger virt-handler-58bwp/debug-20251205-105814...
tar: Removing leading `/' from member names
  ⏳ checking pod harvester-system/virt-handler-t2qlz...
     running debugger virt-handler-t2qlz/debug-20251205-105819...
tar: Removing leading `/' from member names
➡️ version: registry.opensuse.org/isv/rancher/harvester/containers/v1.7/15.7/suse/sles/15.7/virt-launcher:1.6.3
  ⏳ checking pod harvester-system/virt-handler-58bwp...
     running debugger virt-handler-58bwp/debug-20251205-105824...
tar: Removing leading `/' from member names
  ⏳ checking pod harvester-system/virt-handler-t2qlz...
     running debugger virt-handler-t2qlz/debug-20251205-105830...
⚙️ generating report summary...
  📝 output saved to ./out-20251205-105814.tar.gz
```

To compare the host CPU model information generated by the two versions of `virt-launcher`:

```sh
yq '.nodes[] | .[] | "virt_launcher: \(.virt_launcher)\n\(.host_cpu_model)\n"' report.yaml
```

<details>
<summary>Expand to see report output.</summary>

```yaml
global:
  supported_cpu_models:
  - EPYC-Rome-v3: isim-dev2
  - EPYC-Rome-v2: isim-dev2
  - EPYC-Rome-v1: isim-dev2
  - EPYC-Rome-v4: isim-dev2
  - Penryn: isim-dev2 isim-dev2
  - EPYC-v4: isim-dev2
  - EPYC-v1: isim-dev2
  - EPYC-v3: isim-dev2
  - EPYC-v2: isim-dev2
  - Westmere-v1: isim-dev2
  - Westmere-v2: isim-dev2
  - IvyBridge-IBRS: isim-dev2 isim-dev2
  - Denverton-v3: isim-dev2
  - Denverton-v2: isim-dev2
  - EPYC-Rome: isim-dev2 isim-dev2
  - Westmere-IBRS: isim-dev2 isim-dev2
  - Opteron_G3-v1: isim-dev2
  - Nehalem-IBRS: isim-dev2 isim-dev2
  - Dhyana: isim-dev2 isim-dev2
  - Nehalem: isim-dev2 isim-dev2
  - SandyBridge: isim-dev2 isim-dev2
  - IvyBridge: isim-dev2 isim-dev2
  - IvyBridge-v1: isim-dev2
  - IvyBridge-v2: isim-dev2
  - Nehalem-v2: isim-dev2
  - Nehalem-v1: isim-dev2
  - SandyBridge-IBRS: isim-dev2 isim-dev2
  - Westmere: isim-dev2 isim-dev2
  - Penryn-v1: isim-dev2
  - Opteron_G3: isim-dev2 isim-dev2
  - SandyBridge-v2: isim-dev2
  - SandyBridge-v1: isim-dev2
  - EPYC: isim-dev2 isim-dev2
  - EPYC-IBPB: isim-dev2 isim-dev2
  - Dhyana-v2: isim-dev2
  - Dhyana-v1: isim-dev2
nodes:
  isim-dev2:
  - virt_launcher: 1.6.3
    host_cpu_model:
      name: EPYC-Genoa
      vendor: AMD
      required_features:
      - x2apic
      - tsc-deadline
      - hypervisor
      - tsc_adjust
      - spec-ctrl
      - stibp
      - arch-capabilities
      - ssbd
      - cmp_legacy
      - overflow-recov
      - succor
      - virt-ssbd
      - lbrv
      - tsc-scale
      - vmcb-clean
      - flushbyasid
      - pause-filter
      - pfthreshold
      - vgif
      - rdctl-no
      - skip-l1dfl-vmentry
      - mds-no
      - pschange-mc-no
      - gds-no
      - rfds-no
    supported_models:
    - Denverton-v2
    - Denverton-v3
    - Dhyana
    - Dhyana-v1
    - Dhyana-v2
    - EPYC
    - EPYC-IBPB
    - EPYC-Rome
    - EPYC-Rome-v1
    - EPYC-Rome-v2
    - EPYC-Rome-v3
    - EPYC-Rome-v4
    - EPYC-v1
    - EPYC-v2
    - EPYC-v3
    - EPYC-v4
    - IvyBridge
    - IvyBridge-IBRS
    - IvyBridge-v1
    - IvyBridge-v2
    - Nehalem
    - Nehalem-IBRS
    - Nehalem-v1
    - Nehalem-v2
    - Opteron_G3
    - Opteron_G3-v1
    - Penryn
    - Penryn-v1
    - SandyBridge
    - SandyBridge-IBRS
    - SandyBridge-v1
    - SandyBridge-v2
    - Westmere
    - Westmere-IBRS
    - Westmere-v1
    supported_features:
    - 3dnowprefetch
    - abm
    - adx
    - aes
    - amd-psfd
    - amd-ssbd
    - amd-stibp
    - apic
    - arat
    - arch-capabilities
    - avx
    - avx2
    - avx512-bf16
    - avx512-vpopcntdq
    - avx512bitalg
    - avx512bw
    - avx512cd
    - avx512dq
    - avx512f
    - avx512ifma
    - avx512vbmi
    - avx512vbmi2
    - avx512vl
    - avx512vnni
    - bmi1
    - bmi2
    - clflush
    - clflushopt
    - clwb
    - clzero
    - cmov
    - cmp_legacy
    - cr8legacy
    - cx16
    - cx8
    - de
    - erms
    - f16c
    - flushbyasid
    - fma
    - fpu
    - fsgsbase
    - fsrm
    - fxsr
    - fxsr_opt
    - gds-no
    - gfni
    - hypervisor
    - ibpb
    - ibrs
    - invpcid
    - lahf_lm
    - lbrv
    - lfence-always-serializing
    - lm
    - mca
    - mce
    - mds-no
    - misalignsse
    - mmx
    - mmxext
    - movbe
    - msr
    - mtrr
    - no-nested-data-bp
    - npt
    - nrip-save
    - null-sel-clr-base
    - nx
    - osvw
    - overflow-recov
    - pae
    - pat
    - pause-filter
    - pclmuldq
    - pdpe1gb
    - perfctr_core
    - pfthreshold
    - pge
    - pku
    - pni
    - popcnt
    - pschange-mc-no
    - pse
    - pse36
    - rdctl-no
    - rdpid
    - rdrand
    - rdseed
    - rdtscp
    - rfds-no
    - sep
    - sha-ni
    - skip-l1dfl-vmentry
    - smap
    - smep
    - spec-ctrl
    - ssbd
    - sse
    - sse2
    - sse4.1
    - sse4.2
    - sse4a
    - ssse3
    - stibp
    - stibp-always-on
    - succor
    - svm
    - svme-addr-chk
    - syscall
    - tsc
    - tsc-deadline
    - tsc-scale
    - tsc_adjust
    - umip
    - vaes
    - vgif
    - virt-ssbd
    - vmcb-clean
    - vme
    - vpclmulqdq
    - wbnoinvd
    - x2apic
    - xgetbv1
    - xsave
    - xsavec
    - xsaveerptr
    - xsaveopt
    - xsaves
    virsh_version: |
      Compiled against library: libvirt 11.0.0
      Using library: libvirt 11.0.0
      Using API: QEMU 11.0.0
  - virt_launcher: 1.4.1-150600.5.21.2
    host_cpu_model:
      name: EPYC-Genoa
      vendor: AMD
      required_features:
      - x2apic
      - tsc-deadline
      - hypervisor
      - tsc_adjust
      - spec-ctrl
      - stibp
      - arch-capabilities
      - ssbd
      - cmp_legacy
      - virt-ssbd
      - lbrv
      - tsc-scale
      - vmcb-clean
      - flushbyasid
      - pause-filter
      - pfthreshold
      - vgif
      - rdctl-no
      - skip-l1dfl-vmentry
      - mds-no
      - pschange-mc-no
      - gds-no
    supported_models:
    - Westmere-IBRS
    - Westmere
    - SandyBridge-IBRS
    - SandyBridge
    - Penryn
    - Opteron_G3
    - Nehalem-IBRS
    - Nehalem
    - IvyBridge-IBRS
    - IvyBridge
    - EPYC-Rome
    - EPYC-IBPB
    - EPYC
    supported_features:
    - 3dnowprefetch
    - abm
    - adx
    - aes
    - amd-psfd
    - amd-ssbd
    - amd-stibp
    - apic
    - arat
    - arch-capabilities
    - avx
    - avx2
    - avx512-bf16
    - avx512-vpopcntdq
    - avx512bitalg
    - avx512bw
    - avx512cd
    - avx512dq
    - avx512f
    - avx512ifma
    - avx512vbmi
    - avx512vbmi2
    - avx512vl
    - avx512vnni
    - bmi1
    - bmi2
    - clflush
    - clflushopt
    - clwb
    - clzero
    - cmov
    - cmp_legacy
    - cr8legacy
    - cx16
    - cx8
    - de
    - erms
    - f16c
    - flushbyasid
    - fma
    - fpu
    - fsgsbase
    - fsrm
    - fxsr
    - fxsr_opt
    - gds-no
    - gfni
    - hypervisor
    - ibpb
    - ibrs
    - invpcid
    - lahf_lm
    - lbrv
    - lfence-always-serializing
    - lm
    - mca
    - mce
    - mds-no
    - misalignsse
    - mmx
    - mmxext
    - movbe
    - msr
    - mtrr
    - no-nested-data-bp
    - npt
    - nrip-save
    - null-sel-clr-base
    - nx
    - osvw
    - pae
    - pat
    - pause-filter
    - pclmuldq
    - pdpe1gb
    - perfctr_core
    - pfthreshold
    - pge
    - pku
    - pni
    - popcnt
    - pschange-mc-no
    - pse
    - pse36
    - rdctl-no
    - rdpid
    - rdrand
    - rdseed
    - rdtscp
    - sep
    - sha-ni
    - skip-l1dfl-vmentry
    - smap
    - smep
    - spec-ctrl
    - ssbd
    - sse
    - sse2
    - sse4.1
    - sse4.2
    - sse4a
    - ssse3
    - stibp
    - stibp-always-on
    - svm
    - svme-addr-chk
    - syscall
    - tsc
    - tsc-deadline
    - tsc-scale
    - tsc_adjust
    - umip
    - vaes
    - vgif
    - virt-ssbd
    - vmcb-clean
    - vme
    - vpclmulqdq
    - wbnoinvd
    - x2apic
    - xgetbv1
    - xsave
    - xsavec
    - xsaveerptr
    - xsaveopt
    - xsaves
    virsh_version: |
      Compiled against library: libvirt 10.0.0
      Using library: libvirt 10.0.0
      Using API: QEMU 10.0.0
```

</details>

## CPU Capabilities And KubeVirt Node Labeling

This section provides a brief description on the mapping between the reported CPU
capabilities and the node labels managed by the KubeVirt `node-labeller` controller.

The supported CPU models (aka. named models) are gathered from the
`virsh_domcapabilities.xml` file by parsing the non-`host-model` modes for
`usable` models. This is the list of CPU models where all their features are
supported by the node, according to `libvirt`. After removing the obsolete CPU
models from this list, the KubeVirt `node-labeller` controller uses the list to
generate the `cpu-model.node.kubevirt.io` labels on the nodes<sup>[ref][3]</sup>.

The CPU host model name, vendor and required features are parsed from the
`host-model` mode section in the `virsh_domcapabilities.xml` file. Features
identified with the `require` policy are deemed as required by the host CPU model.
According to `libvirt`, this model is the closest match to the host CPU from the
list of supported models. They are translated to the
`host-model-cpu.node.kubevirt.io`, `cpu-vendor.node.kubevirt.io` and
`host-model-required-features.node.kubevirt.io` labels on the nodes<sup>[ref][2]</sup>.

During live migration, KubeVirt updates the `virt-launcher` pods of virtual
machines that don't have specific CPU model and features defined, with the host
CPU model and its required features as the pod's node selectors.

In contrast, virtual machines that have specific CPU model defined in its
specification ends up with `virt-launcher` pod that uses the named model as its
node selector.

It is possible for a CPU model to appear in the `host-model-cpu.node.kubevirt.io`
label, but not in the `cpu-model.node.kubevirt.io` label because the node may not
support all the required features of that model<sup>[ref][5]</sup>.

The known CPU models are the superset of the supported CPU models, which include
non-`usable` models. Obsolete CPU models are excluded from this list to form the
list of CPUs in the `cpu-model-migration.node.kubevirt.io` labels on the
nodes<sup>[ref][4]</sup>. These labels indicate that a node is a potential
migration target for virtual machines using the specified host CPU model only if
all the features required by the host model are supported on the node.

The supported features are gathered from the `supported_features.xml` files.
These are features marked with the `require` policy. They are translated to the
`cpu-feature.node.kubevirt.io` label<sup>[ref][1]</sup></sup>.

[1]: https://github.com/kubevirt/kubevirt/blob/de16a73f4ea3e48a5c8796d9db508b49960417bb/pkg/virt-handler/node-labeller/node_labeller.go#L241-L245
[2]: https://github.com/kubevirt/kubevirt/blob/de16a73f4ea3e48a5c8796d9db508b49960417bb/pkg/virt-handler/node-labeller/node_labeller.go#L279-L284
[3]: https://github.com/kubevirt/kubevirt/blob/de16a73f4ea3e48a5c8796d9db508b49960417bb/pkg/virt-handler/node-labeller/node_labeller.go#L248-L250
[4]: https://github.com/kubevirt/kubevirt/blob/de16a73f4ea3e48a5c8796d9db508b49960417bb/pkg/virt-handler/node-labeller/node_labeller.go#L251-L253
[5]: https://github.com/kubevirt/kubevirt/issues/14813

## How It Works

The script starts an ephemeral `virt-launcher` container in each `virt-handler`
pod to collect the host CPU capabilities information of the node. Essentially, it
replicates the task performed by the `node-labeller` init container in real time,
without launching a new pod.

If the optional `-i` argument is specified, the script uses its value as the
image of the ephemeral container. This allows us to compare the information
generated by different versions of `virt-launcher`, libvirt and QEMU.

Without the `-i` argument, the ephemeral container uses the same image as the
`node-labeller` init container.

The container executes the KubeVirt `node-labeller.sh` script<sup>[ref][6]</sup>, writes
the output XML files to the container's `/var/lib/kubevirt-node-labeller` folder,
and copies the output from the container to your shell. It operates within the
security context boundary of its owner `virt-handler` pod.

[6]: https://github.com/kubevirt/kubevirt/blob/de16a73f4ea3e48a5c8796d9db508b49960417bb/cmd/virt-launcher/node-labeller/node-labeller.sh

![virt-handler pod without debug containers](./img/virt-handler-without-debug-containers.excalidraw.png)
![virt-handler pod with debug containers](./img/virt-handler-with-debug-containers.excalidraw.png)

To check the state of the ephemeral containers in all the `virt-handler` pods:

```sh
kubectl -n harvester-system get po -lkubevirt.io=virt-handler -ojsonpath='{.items[*].status.ephemeralContainerStatuses}' | jq -r '.[] | select(.name | test("debug-")) | "Name: \(.name), State: \(.state)"'
```

```sh
Name: debug-20251203-093008, State: {"running":{"startedAt":"2025-12-03T17:30:08Z"}}
```

To view their logs:

```sh
kubectl -n harvester-system logs virt-handler-t2qlz -c debug-20251205-105830
```

```sh
++ uname -m
+ ARCH=x86_64
+ MACHINE=q35
+ '[' x86_64 == aarch64 ']'
+ '[' x86_64 == s390x ']'
+ '[' x86_64 '!=' x86_64 ']'
+ set +o pipefail
++ grep -w kvm /proc/misc
++ cut -f 1 '-d '
+ KVM_MINOR=232
+ set -o pipefail
+ VIRTTYPE=qemu
+ '[' '!' -e /dev/kvm ']'
+ '[' -e /dev/kvm ']'
+ chmod o+rw /dev/kvm
+ VIRTTYPE=kvm
+ '[' -e /dev/sev ']'
+ virtqemud -d
+ virsh domcapabilities --machine q35 --arch x86_64 --virttype kvm
+ '[' x86_64 == x86_64 ']'
+ virsh domcapabilities --machine q35 --arch x86_64 --virttype kvm
+ virsh hypervisor-cpu-baseline --features /dev/stdin --machine q35 --arch x86_64 --virttype kvm
+ virsh capabilities
```

The ephemeral container has a default TTL of 1 hour. It does not interfere with
the operation of the primary container and is automatically removed when the
owner pod is restarted.

## ToDo

* Account for known CPU models list

## LICENSE

Apache License Version 2.0.

See [LICENSE](LICENSE).
