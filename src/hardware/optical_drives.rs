#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct OpticalDriveInfo {
    pub name: String,
    pub vendor: Option<String>,
    pub kind: String,
}

/// Infaillible par design : absence de lecteur ou erreur d'accès matériel
/// renvoient simplement un `Vec` vide.
pub fn collect() -> Vec<OpticalDriveInfo> {
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
