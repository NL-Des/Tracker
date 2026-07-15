use super::WifiNetworkInfo;

fn parse_field(output: &str, key: &str) -> Option<String> {
    output.lines().find_map(|line| {
        let (k, v) = line.split_once(':')?;
        if k.trim() == key {
            Some(v.trim().to_string())
        } else {
            None
        }
    })
}

/// `netsh wlan show interfaces` ne nécessite pas de droits admin en lecture.
pub fn collect() -> Vec<WifiNetworkInfo> {
    let Some(text) = crate::command::run("netsh", &["wlan", "show", "interfaces"]) else {
        return Vec::new();
    };

    let Some(ssid) = parse_field(&text, "SSID") else {
        return Vec::new();
    };
    let signal_percent = parse_field(&text, "Signal")
        .and_then(|v| v.trim_end_matches('%').parse::<i32>().ok());
    let interface = parse_field(&text, "Name");
    let link_rate_mbps = parse_field(&text, "Receive rate (Mbps)")
        .and_then(|v| v.parse::<f64>().ok());

    vec![WifiNetworkInfo {
        ssid,
        signal_percent,
        interface,
        link_rate_mbps,
    }]
}
