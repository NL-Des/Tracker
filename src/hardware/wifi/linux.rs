use super::WifiNetworkInfo;

/// `nmcli` liste les connexions Wi-Fi actives sans élévation de privilèges.
/// Format `-t` (terse) séparé par `:` : `active:ssid:signal:device:rate`.
/// Le champ `rate` est au format "270 Mbit/s".
pub fn collect() -> Vec<WifiNetworkInfo> {
    let Some(text) = crate::command::run(
        "nmcli",
        &["-t", "-f", "active,ssid,signal,device,rate", "dev", "wifi"],
    ) else {
        return Vec::new();
    };

    text.lines()
        .filter_map(|line| {
            let fields: Vec<&str> = line.split(':').collect();
            let [active, ssid, signal, device, rate] = fields.as_slice() else {
                return None;
            };
            if *active != "yes" || ssid.is_empty() {
                return None;
            }
            let link_rate_mbps = rate
                .split_whitespace()
                .next()
                .and_then(|v| v.parse::<f64>().ok());
            Some(WifiNetworkInfo {
                ssid: ssid.to_string(),
                signal_percent: signal.parse().ok(),
                interface: Some(device.to_string()),
                link_rate_mbps,
            })
        })
        .collect()
}
