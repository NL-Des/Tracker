use super::WifiNetworkInfo;

/// `nmcli` liste les connexions Wi-Fi actives sans élévation de privilèges.
/// Format `-t` (terse) séparé par `:` : `active:ssid:signal:device`.
pub fn collect() -> Vec<WifiNetworkInfo> {
    let Some(text) = crate::command::run(
        "nmcli",
        &["-t", "-f", "active,ssid,signal,device", "dev", "wifi"],
    ) else {
        return Vec::new();
    };

    text.lines()
        .filter_map(|line| {
            let fields: Vec<&str> = line.split(':').collect();
            let [active, ssid, signal, device] = fields.as_slice() else {
                return None;
            };
            if *active != "yes" || ssid.is_empty() {
                return None;
            }
            Some(WifiNetworkInfo {
                ssid: ssid.to_string(),
                signal_percent: signal.parse().ok(),
                interface: Some(device.to_string()),
            })
        })
        .collect()
}
