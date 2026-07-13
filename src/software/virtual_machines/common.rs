use super::VirtualMachineInfo;

/// `VBoxManage` a une surface identique sur Linux/macOS/Windows.
pub fn collect_virtualbox() -> Vec<VirtualMachineInfo> {
    let Some(list) = crate::command::run("VBoxManage", &["list", "vms"]) else {
        return Vec::new();
    };
    let running = crate::command::run("VBoxManage", &["list", "runningvms"]).unwrap_or_default();

    list.lines()
        .filter_map(|line| {
            // format de ligne : "nom-vm" {uuid}
            let name = line.split('"').nth(1)?.to_string();
            let uuid = line.split('{').nth(1)?.split('}').next()?.to_string();
            let state = if running.contains(&uuid) { "running" } else { "stopped" }.to_string();
            Some(VirtualMachineInfo {
                name,
                hypervisor: "VirtualBox".to_string(),
                state,
                identifier: Some(uuid),
            })
        })
        .collect()
}
