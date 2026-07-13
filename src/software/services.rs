#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct ServiceInfo {
    pub name: String,
    pub status: String,
}

/// Infaillible par design : absence d'outil ou erreur d'accès renvoient
/// simplement un `Vec` vide. Lecture seule, aucune élévation requise.
pub fn collect() -> Vec<ServiceInfo> {
    crate::os_dispatch::dispatch_os!(linux::collect(), macos::collect(), windows::collect(), Vec::new())
}
