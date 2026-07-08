use super::AutostartEntryInfo;
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

/// Clé de registre `HKEY_CURRENT_USER\...\Run`, lecture libre sans droits
/// admin (contrairement à la clé équivalente sous `HKEY_LOCAL_MACHINE`).
pub fn collect() -> Vec<AutostartEntryInfo> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let Ok(run_key) = hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run") else {
        return Vec::new();
    };

    run_key
        .enum_values()
        .filter_map(|entry| entry.ok())
        .map(|(name, value)| AutostartEntryInfo {
            name,
            command: Some(value.to_string()),
        })
        .collect()
}
