#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;
use std::path::Path;

#[derive(Serialize)]
pub struct BrowserExtensionInfo {
    pub id: String,
    pub name: String,
    pub version: String,
}

#[derive(Serialize)]
pub struct BrowserInfo {
    pub name: String,
    pub version: Option<String>,
    pub path: Option<String>,
    pub is_default: bool,
    /// Non peuplé pour l'instant (hors scope actuel) : la struct reste prête
    /// pour une future extension "extensions installées" sans casser le schéma JSON.
    pub extensions: Option<Vec<BrowserExtensionInfo>>,
}

pub fn collect() -> Vec<BrowserInfo> {
    crate::os_dispatch::dispatch_os!(linux::collect(), macos::collect(), windows::collect(), Vec::new())
}

/// Tente d'obtenir la version d'un exécutable en le lançant avec `--version`.
/// Partagé entre les backends Linux/Windows/macOS : la plupart des navigateurs
/// (Chrome, Chromium, Edge, Brave, Opera, Firefox récents) supportent ce flag.
#[allow(dead_code)]
pub(crate) fn try_get_version(exe_path: &Path) -> Option<String> {
    let text = crate::command::run_lenient_stdout_or_stderr(exe_path.to_str()?, &["--version"])?;
    let trimmed = text.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}
