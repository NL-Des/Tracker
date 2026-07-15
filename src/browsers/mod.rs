#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

mod extensions;

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
    /// Extensions installées, lues dans le profil par défaut de
    /// l'utilisateur courant (lecture seule, aucune élévation requise).
    /// `None` si le profil n'a pas pu être localisé.
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
