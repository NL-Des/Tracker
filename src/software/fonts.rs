#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(any(target_os = "linux", target_os = "macos"))]
mod common;

use serde::Serialize;

#[derive(Serialize)]
pub struct FontsSummary {
    pub total_count: usize,
    pub families: Vec<String>,
}

/// Infaillible par design : absence d'outil (`fc-list`...) ou erreur d'accès
/// renvoient une liste vide. Lecture seule, aucune élévation requise.
pub fn collect() -> FontsSummary {
    let mut families: Vec<String> =
        crate::os_dispatch::dispatch_os!(linux::collect(), macos::collect(), windows::collect(), Vec::new());
    families.sort();
    families.dedup();
    FontsSummary {
        total_count: families.len(),
        families,
    }
}
