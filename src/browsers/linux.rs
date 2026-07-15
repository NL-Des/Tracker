use super::{extensions, try_get_version, BrowserExtensionInfo, BrowserInfo};
use std::path::PathBuf;

/// Dossier de profil Chromium par défaut sous `~/.config/<dir>/Default`, par
/// nom affiché de navigateur.
const CHROMIUM_CONFIG_DIRS: &[(&str, &str)] = &[
    ("Google Chrome", "google-chrome"),
    ("Chromium", "chromium"),
    ("Brave", "BraveSoftware/Brave-Browser"),
    ("Microsoft Edge", "microsoft-edge"),
    ("Opera", "opera"),
    ("Vivaldi", "vivaldi"),
];

/// Localise le profil Firefox par défaut via `profiles.ini` : privilégie la
/// clé `Default=` d'une section `[Install...]` (format moderne), sinon la
/// première clé `Path=` trouvée (format historique). Best-effort : plusieurs
/// profils sont possibles, on ne retient que le profil par défaut.
fn firefox_profile_dir(home: &str) -> Option<PathBuf> {
    let base = PathBuf::from(home).join(".mozilla/firefox");
    let text = std::fs::read_to_string(base.join("profiles.ini")).ok()?;

    let install_default = {
        let mut in_install_section = false;
        text.lines().find_map(|line| {
            let line = line.trim();
            if line.starts_with('[') {
                in_install_section = line.starts_with("[Install");
                return None;
            }
            if in_install_section {
                line.strip_prefix("Default=").map(|s| s.to_string())
            } else {
                None
            }
        })
    };

    let path = install_default
        .or_else(|| text.lines().find_map(|l| l.trim().strip_prefix("Path=").map(|s| s.to_string())))?;
    Some(base.join(path))
}

fn read_extensions(display_name: &str, home: Option<&str>) -> Option<Vec<BrowserExtensionInfo>> {
    let home = home?;
    if display_name == "Mozilla Firefox" {
        let profile_dir = firefox_profile_dir(home)?;
        return Some(extensions::read_firefox_extensions(&profile_dir));
    }
    let (_, config_dir) = CHROMIUM_CONFIG_DIRS
        .iter()
        .find(|(name, _)| *name == display_name)?;
    let profile_dir = PathBuf::from(home).join(".config").join(config_dir).join("Default");
    Some(extensions::read_chromium_extensions(&profile_dir))
}

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
    let path = crate::command::run("which", &[binary])?;
    let path = path.trim();
    if path.is_empty() {
        None
    } else {
        Some(PathBuf::from(path))
    }
}

fn default_browser_desktop_id() -> Option<String> {
    let text = crate::command::run("xdg-settings", &["get", "default-web-browser"])?;
    let text = text.trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}

pub fn collect() -> Vec<BrowserInfo> {
    let default_desktop_id = default_browser_desktop_id();
    let home = std::env::var("HOME").ok();

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
                    extensions: read_extensions(display_name, home.as_deref()),
                })
            })
        })
        .collect()
}
