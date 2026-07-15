#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct WifiNetworkInfo {
    pub ssid: String,
    pub signal_percent: Option<i32>,
    pub interface: Option<String>,
    /// Débit de liaison négocié (Mbps), pas un débit mesuré. Lecture libre,
    /// aucune élévation requise sur les 3 OS.
    pub link_rate_mbps: Option<f64>,
}

/// Infaillible par design : absence d'interface Wi-Fi active, outil non
/// installé, ou erreur d'accès renvoient simplement un `Vec` vide.
pub fn collect() -> Vec<WifiNetworkInfo> {
    crate::os_dispatch::dispatch_os!(linux::collect(), macos::collect(), windows::collect(), Vec::new())
}
