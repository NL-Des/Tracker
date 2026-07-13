#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct PrinterInfo {
    pub name: String,
    /// "Imprimante" ou "Scanner".
    pub kind: String,
}

/// Infaillible par design : absence d'imprimante/scanner ou erreur d'accès
/// renvoient simplement un `Vec` vide.
pub fn collect() -> Vec<PrinterInfo> {
    crate::os_dispatch::dispatch_os!(linux::collect(), macos::collect(), windows::collect(), Vec::new())
}
