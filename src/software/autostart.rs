#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct AutostartEntryInfo {
    pub name: String,
    pub command: Option<String>,
}

/// Infaillible par design : liste vide si aucun répertoire d'autostart
/// utilisateur n'existe ou n'est lisible. Portée utilisateur uniquement,
/// aucune élévation requise.
pub fn collect() -> Vec<AutostartEntryInfo> {
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
