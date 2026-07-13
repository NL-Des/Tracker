#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct UsbDeviceInfo {
    pub name: String,
    pub vendor: Option<String>,
}

/// Infaillible par design : absence de périphérique ou erreur d'accès
/// matériel renvoient simplement un `Vec` vide. Pas de classification fine
/// (stockage/réseau/autre) : nécessiterait de parser les descripteurs
/// d'interface USB, jugé hors scope pour un inventaire.
pub fn collect() -> Vec<UsbDeviceInfo> {
    crate::os_dispatch::dispatch_os!(linux::collect(), macos::collect(), windows::collect(), Vec::new())
}
