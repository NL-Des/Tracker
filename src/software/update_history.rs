#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct UpdateHistoryEntryInfo {
    pub date: String,
    pub description: String,
}

/// Limite volontaire du nombre d'entrées renvoyées pour éviter un rapport
/// démesurément long (l'historique complet peut compter des milliers de
/// lignes sur une machine ancienne).
const MAX_ENTRIES: usize = 20;

/// Infaillible par design : logs absents ou illisibles (droits de fichier)
/// renvoient simplement un `Vec` vide.
pub fn collect() -> Vec<UpdateHistoryEntryInfo> {
    crate::os_dispatch::dispatch_os!(linux::collect(), macos::collect(), windows::collect(), Vec::new())
}
