#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct PciDeviceInfo {
    pub name: String,
    pub class: String,
}

/// Infaillible par design : absence d'outil (`lspci`...) ou erreur d'accès
/// renvoient simplement un `Vec` vide.
pub fn collect() -> Vec<PciDeviceInfo> {
    crate::os_dispatch::dispatch_os!(linux::collect(), macos::collect(), windows::collect(), Vec::new())
}
