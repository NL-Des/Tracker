#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;
mod common;

use serde::Serialize;

#[derive(Serialize)]
pub struct VirtualMachineInfo {
    pub name: String,
    pub hypervisor: String,
    pub state: String,
    pub identifier: Option<String>,
}

/// Infaillible par design : absence d'outil (VBoxManage/virsh) ou erreur
/// d'accès renvoient simplement un `Vec` vide. Lecture seule, aucune
/// élévation requise.
pub fn collect() -> Vec<VirtualMachineInfo> {
    crate::os_dispatch::dispatch_os!(linux::collect(), macos::collect(), windows::collect(), Vec::new())
}
