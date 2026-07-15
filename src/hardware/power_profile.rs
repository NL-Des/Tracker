#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct PowerProfileInfo {
    pub profile: Option<String>,
    pub sleep_mode: Option<String>,
}

/// Infaillible par design : absence d'outil (`powerprofilesctl`/`pmset`/
/// `powercfg`) ou erreur d'accès renvoient des champs `None`. Interroger ces
/// outils est en lecture seule ; seule leur *modification* nécessiterait une
/// élévation (non faite ici).
pub fn collect() -> PowerProfileInfo {
    crate::os_dispatch::dispatch_os!(
        linux::collect(),
        macos::collect(),
        windows::collect(),
        PowerProfileInfo { profile: None, sleep_mode: None }
    )
}
