#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct InstalledAppInfo {
    pub name: String,
    pub version: Option<String>,
    pub publisher: Option<String>,
    /// Méthode de détection utilisée (ex: "desktop-file", "registry", "app-bundle").
    pub source: String,
}

pub fn collect() -> Vec<InstalledAppInfo> {
    crate::os_dispatch::dispatch_os!(linux::collect(), macos::collect(), windows::collect(), Vec::new())
}
