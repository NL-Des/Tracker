use super::WifiNetworkInfo;
use std::process::Command;

const AIRPORT_PATH: &str =
    "/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport";

fn parse_field(output: &str, key: &str) -> Option<String> {
    output.lines().find_map(|line| {
        let line = line.trim();
        line.strip_prefix(key)?.strip_prefix(':').map(|v| v.trim().to_string())
    })
}

pub fn collect() -> Vec<WifiNetworkInfo> {
    let Ok(output) = Command::new(AIRPORT_PATH).arg("-I").output() else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(&output.stdout);

    let Some(ssid) = parse_field(&text, "SSID") else {
        return Vec::new();
    };
    let signal_percent = parse_field(&text, "agrCtlRSSI")
        .and_then(|v| v.parse::<i32>().ok())
        // RSSI en dBm (~ -30 excellent, -90 très faible) converti en % grossier.
        .map(|rssi| ((rssi + 90) * 100 / 60).clamp(0, 100));

    vec![WifiNetworkInfo {
        ssid,
        signal_percent,
        interface: None,
    }]
}
