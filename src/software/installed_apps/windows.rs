use super::InstalledAppInfo;
use std::collections::HashSet;
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
use winreg::RegKey;

fn collect_from(
    root: &RegKey,
    subkey: &str,
    apps: &mut Vec<InstalledAppInfo>,
    seen: &mut HashSet<String>,
) {
    let Ok(uninstall) = root.open_subkey(subkey) else {
        return;
    };
    for key_name in uninstall.enum_keys().filter_map(|k| k.ok()) {
        let Ok(entry) = uninstall.open_subkey(&key_name) else {
            continue;
        };
        let Ok(name) = entry.get_value::<String, _>("DisplayName") else {
            continue;
        };
        if !seen.insert(name.clone()) {
            continue;
        }
        let version: Option<String> = entry.get_value("DisplayVersion").ok();
        let publisher: Option<String> = entry.get_value("Publisher").ok();
        apps.push(InstalledAppInfo {
            name,
            version,
            publisher,
            source: "registry".to_string(),
        });
    }
}

pub fn collect() -> Vec<InstalledAppInfo> {
    let mut apps = Vec::new();
    let mut seen = HashSet::new();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    collect_from(
        &hklm,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        &mut apps,
        &mut seen,
    );
    collect_from(
        &hklm,
        r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
        &mut apps,
        &mut seen,
    );
    collect_from(
        &hkcu,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        &mut apps,
        &mut seen,
    );

    apps
}
