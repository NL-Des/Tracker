use super::{extensions, try_get_version, BrowserExtensionInfo, BrowserInfo};
use std::path::PathBuf;
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
use winreg::RegKey;

/// Dossier de profil Chromium par défaut sous
/// `%LOCALAPPDATA%\<dir>\User Data\Default`, par nom de clé de registre
/// `StartMenuInternet` (ex. "Google Chrome", "Microsoft Edge").
const CHROMIUM_LOCALAPPDATA_DIRS: &[(&str, &str)] = &[
    ("Google Chrome", r"Google\Chrome"),
    ("Microsoft Edge", r"Microsoft\Edge"),
    ("Brave", r"BraveSoftware\Brave-Browser"),
    ("Opera", r"Opera Software\Opera Stable"),
    ("Vivaldi", r"Vivaldi"),
];

/// Best-effort : prend le premier profil trouvé (souvent le seul en
/// pratique) plutôt que de parser `profiles.ini` comme sur Linux.
fn firefox_profile_dir(appdata: &str) -> Option<PathBuf> {
    let profiles_dir = PathBuf::from(appdata).join(r"Mozilla\Firefox\Profiles");
    let entries = std::fs::read_dir(profiles_dir).ok()?;
    entries
        .filter_map(|e| e.ok())
        .find(|e| e.path().is_dir())
        .map(|e| e.path())
}

fn read_extensions(key_name: &str) -> Option<Vec<BrowserExtensionInfo>> {
    if key_name.to_lowercase().contains("firefox") {
        let appdata = std::env::var("APPDATA").ok()?;
        let profile_dir = firefox_profile_dir(&appdata)?;
        return Some(extensions::read_firefox_extensions(&profile_dir));
    }
    let local_appdata = std::env::var("LOCALAPPDATA").ok()?;
    let (_, dir) = CHROMIUM_LOCALAPPDATA_DIRS
        .iter()
        .find(|(name, _)| key_name.to_lowercase().contains(&name.to_lowercase()))?;
    let profile_dir = PathBuf::from(local_appdata).join(dir).join(r"User Data\Default");
    Some(extensions::read_chromium_extensions(&profile_dir))
}

fn default_browser_prog_id() -> Option<String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu
        .open_subkey(
            r"SOFTWARE\Microsoft\Windows\Shell\Associations\UrlAssociations\http\UserChoice",
        )
        .ok()?;
    key.get_value("ProgId").ok()
}

fn prog_id_matches(prog_id: &str, key_name: &str) -> bool {
    let prog_id = prog_id.to_lowercase();
    let key_name = key_name.to_lowercase();
    prog_id.contains(&key_name) || key_name.contains(&prog_id)
}

pub fn collect() -> Vec<BrowserInfo> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let default_prog_id = default_browser_prog_id();

    let Ok(clients) = hklm.open_subkey(r"SOFTWARE\Clients\StartMenuInternet") else {
        return Vec::new();
    };

    clients
        .enum_keys()
        .filter_map(|key_name| key_name.ok())
        .filter_map(|key_name| {
            let sub = clients.open_subkey(&key_name).ok()?;
            let display_name: String = sub.get_value("").unwrap_or_else(|_| key_name.clone());
            let command_key = sub.open_subkey(r"shell\open\command").ok()?;
            let command: String = command_key.get_value("").ok()?;
            let exe_path = command.trim_matches('"').to_string();
            let version = try_get_version(&PathBuf::from(&exe_path));
            let is_default = default_prog_id
                .as_deref()
                .map(|prog_id| prog_id_matches(prog_id, &key_name))
                .unwrap_or(false);

            Some(BrowserInfo {
                name: display_name,
                version,
                path: Some(exe_path),
                is_default,
                extensions: read_extensions(&key_name),
            })
        })
        .collect()
}
