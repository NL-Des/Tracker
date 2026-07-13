#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct NetworkConnectionInfo {
    pub protocol: String,
    pub local_address: String,
    pub state: String,
}

/// Infaillible par design : ne liste que les connexions de l'utilisateur
/// courant (la liste complète tous utilisateurs peut nécessiter root selon
/// l'OS), aucune élévation requise pour ce sous-ensemble.
pub fn collect() -> Vec<NetworkConnectionInfo> {
    crate::os_dispatch::dispatch_os!(linux::collect(), macos::collect(), windows::collect(), Vec::new())
}
