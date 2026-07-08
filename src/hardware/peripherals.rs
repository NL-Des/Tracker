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
    #[cfg(target_os = "linux")]
    {
        linux::collect()
    }
    #[cfg(target_os = "windows")]
    {
        windows::collect()
    }
    #[cfg(target_os = "macos")]
    {
        macos::collect()
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        Vec::new()
    }
}
