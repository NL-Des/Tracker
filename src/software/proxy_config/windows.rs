use super::ProxyConfigInfo;
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

/// `HKEY_CURRENT_USER\...\Internet Settings` : lecture seule, aucune
/// élévation requise (contrairement à la clé équivalente sous
/// `HKEY_LOCAL_MACHINE`).
pub fn collect() -> Option<ProxyConfigInfo> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu
        .open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings")
        .ok()?;

    let enabled: u32 = key.get_value("ProxyEnable").unwrap_or(0);
    if enabled == 0 {
        return None;
    }

    let server: String = key.get_value("ProxyServer").ok()?;
    let (http_proxy, https_proxy) = if server.contains('=') {
        let mut http = None;
        let mut https = None;
        for part in server.split(';') {
            if let Some(v) = part.strip_prefix("http=") {
                http = Some(v.to_string());
            } else if let Some(v) = part.strip_prefix("https=") {
                https = Some(v.to_string());
            }
        }
        (http, https)
    } else {
        (Some(server.clone()), Some(server))
    };

    Some(ProxyConfigInfo {
        http_proxy,
        https_proxy,
        no_proxy: key.get_value("ProxyOverride").ok(),
        source: "registry".to_string(),
    })
}
