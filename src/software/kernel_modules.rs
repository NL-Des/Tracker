#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct KernelModuleInfo {
    pub name: String,
    pub size_bytes: u64,
}

/// Infaillible par design : absence d'outil ou erreur d'accès renvoient
/// simplement un `Vec` vide.
pub fn collect() -> Vec<KernelModuleInfo> {
    crate::os_dispatch::dispatch_os!(linux::collect(), macos::collect(), windows::collect(), Vec::new())
}
