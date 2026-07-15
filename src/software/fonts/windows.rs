use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

/// Énumération des valeurs de la clé de registre des polices installées
/// (lecture seule, `HKEY_LOCAL_MACHINE` en lecture ne nécessite pas admin).
/// Les noms de valeur ont typiquement la forme "Arial (TrueType)".
pub fn collect() -> Vec<String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let Ok(fonts_key) = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Fonts") else {
        return Vec::new();
    };

    fonts_key
        .enum_values()
        .filter_map(|entry| entry.ok())
        .map(|(name, _value)| name.split('(').next().unwrap_or(&name).trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
