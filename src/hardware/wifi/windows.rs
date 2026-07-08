use super::WifiNetworkInfo;
use std::process::Command;

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
    let Ok(output) = Command::new("netsh")
        .args(["wlan", "show", "interfaces"])
        .output()
    else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(&output.stdout);

    let Some(ssid) = parse_field(&text, "SSID") else {
        return Vec::new();
    };
    let signal_percent = parse_field(&text, "Signal")
        .and_then(|v| v.trim_end_matches('%').parse::<i32>().ok());
    let interface = parse_field(&text, "Name");

    vec![WifiNetworkInfo {
        ssid,
        signal_percent,
        interface,
    }]
}
