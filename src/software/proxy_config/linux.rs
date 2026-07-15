use super::ProxyConfigInfo;

/// `gsettings` (GNOME) est en lecture libre ; absent sur les environnements
/// sans GNOME (auquel cas `None`, comme pour toute config non détectée).
pub fn collect() -> Option<ProxyConfigInfo> {
    let mode = crate::command::run("gsettings", &["get", "org.gnome.system.proxy", "mode"])?;
    if !mode.trim().contains("manual") {
        return None;
    }

    let host = crate::command::run("gsettings", &["get", "org.gnome.system.proxy.http", "host"])
        .map(|s| s.trim().trim_matches('\'').to_string());
    let port = crate::command::run("gsettings", &["get", "org.gnome.system.proxy.http", "port"])
        .map(|s| s.trim().to_string());

    let http_proxy = match (host, port) {
        (Some(h), Some(p)) if !h.is_empty() => Some(format!("{h}:{p}")),
        _ => None,
    };

    Some(ProxyConfigInfo {
        http_proxy,
        https_proxy: None,
        no_proxy: None,
        source: "gsettings".to_string(),
    })
}
