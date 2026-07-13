#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct CameraInfo {
    pub name: String,
}

/// Infaillible par design : absence de caméra ou erreur d'accès matériel
/// renvoient simplement un `Vec` vide.
pub fn collect() -> Vec<CameraInfo> {
    crate::os_dispatch::dispatch_os!(linux::collect(), macos::collect(), windows::collect(), Vec::new())
}
