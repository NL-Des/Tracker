#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct FanInfo {
    pub name: String,
    /// Régime en tr/min, si le driver l'expose (absent sur beaucoup de
    /// laptops). Pas de champ marque/modèle : cette info vit dans les
    /// tables SMBIOS (type 27), lues via `dmidecode`, inaccessible sans
    /// droits root (comme l'UUID machine, cf. `report.rs`).
    pub speed_rpm: Option<u32>,
}

/// Infaillible par design : absence de ventilateur détectable ou erreur
/// d'accès matériel renvoient simplement un `Vec` vide.
pub fn collect() -> Vec<FanInfo> {
    crate::os_dispatch::dispatch_os!(linux::collect(), macos::collect(), windows::collect(), Vec::new())
}
