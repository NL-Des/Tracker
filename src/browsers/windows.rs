use super::{try_get_version, BrowserInfo};
use std::path::PathBuf;
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
use winreg::RegKey;

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
                extensions: None,
            })
        })
        .collect()
}
