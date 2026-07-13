use super::common::collect_virtualbox;
use super::VirtualMachineInfo;

pub fn collect() -> Vec<VirtualMachineInfo> {
    let mut vms = collect_virtualbox();
    vms.extend(collect_libvirt());
    vms
}

/// `virsh list --all` liste les domaines libvirt/QEMU-KVM définis, qu'ils
/// soient en cours d'exécution ou arrêtés.
fn collect_libvirt() -> Vec<VirtualMachineInfo> {
    let Some(text) = crate::command::run("virsh", &["list", "--all"]) else {
        return Vec::new();
    };

    text.lines()
        .skip(2) // ligne d'en-tête + séparateur
        .filter_map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() < 3 {
                return None;
            }
            let id = fields[0];
            let name = fields[1].to_string();
            let state = fields[2..].join(" "); // ex: "shut off" (multi-mots)
            Some(VirtualMachineInfo {
                name,
                hypervisor: "QEMU/KVM".to_string(),
                state,
                identifier: if id == "-" { None } else { Some(id.to_string()) },
            })
        })
        .collect()
}
