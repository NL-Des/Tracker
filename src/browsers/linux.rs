use super::{try_get_version, BrowserInfo};
use std::path::PathBuf;
use std::process::Command;

const KNOWN_BROWSERS: &[(&str, &[&str])] = &[
    ("Google Chrome", &["google-chrome-stable", "google-chrome"]),
    ("Chromium", &["chromium", "chromium-browser"]),
    ("Mozilla Firefox", &["firefox", "firefox-esr"]),
    ("Brave", &["brave-browser", "brave"]),
    (
        "Microsoft Edge",
        &["microsoft-edge-stable", "microsoft-edge"],
    ),
    ("Opera", &["opera"]),
    ("Vivaldi", &["vivaldi-stable", "vivaldi"]),
];

/// Best-effort : utilisé uniquement pour renseigner le chemin de l'exécutable
/// dans le rapport, la détection de présence se fait via `try_get_version`.
fn resolve_path(binary: &str) -> Option<PathBuf> {
    let output = Command::new("which").arg(binary).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if path.is_empty() {
        None
    } else {
        Some(PathBuf::from(path))
    }
}

fn default_browser_desktop_id() -> Option<String> {
    let output = Command::new("xdg-settings")
        .args(["get", "default-web-browser"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

pub fn collect() -> Vec<BrowserInfo> {
    let default_desktop_id = default_browser_desktop_id();

    KNOWN_BROWSERS
        .iter()
        .filter_map(|(display_name, binaries)| {
            binaries.iter().find_map(|&binary| {
                let version = try_get_version(&PathBuf::from(binary))?;
                let is_default = default_desktop_id
                    .as_deref()
                    .map(|id| id.to_lowercase().contains(&binary.to_lowercase()))
                    .unwrap_or(false);

                Some(BrowserInfo {
                    name: display_name.to_string(),
                    version: Some(version),
                    path: resolve_path(binary).map(|p| p.to_string_lossy().to_string()),
                    is_default,
                    extensions: None,
                })
            })
        })
        .collect()
}
