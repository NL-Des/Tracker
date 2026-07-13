#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct PeripheralInfo {
    pub name: String,
    /// "Clavier", "Enceintes", ...
    pub kind: String,
}

/// Infaillible par design : absence de périphérique ou erreur d'accès
/// matériel renvoient simplement un `Vec` vide.
pub fn collect() -> Vec<PeripheralInfo> {
    crate::os_dispatch::dispatch_os!(linux::collect(), macos::collect(), windows::collect(), Vec::new())
}
