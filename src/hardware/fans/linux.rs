use super::FanInfo;
use std::fs;

/// `fanN_input` (tr/min) est lisible sans root quand le driver l'expose.
fn hwmon_fans() -> Vec<FanInfo> {
    let mut fans = Vec::new();
    let Ok(chips) = fs::read_dir("/sys/class/hwmon") else {
        return fans;
    };

    for chip in chips.filter_map(|e| e.ok()) {
        let chip_name = fs::read_to_string(chip.path().join("name"))
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| chip.file_name().to_string_lossy().to_string());

        let Ok(entries) = fs::read_dir(chip.path()) else {
            continue;
        };
        for entry in entries.filter_map(|e| e.ok()) {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if !file_name.ends_with("_input") || !file_name.starts_with("fan") {
                continue;
            }
            let Ok(rpm) = fs::read_to_string(entry.path())
                .unwrap_or_default()
                .trim()
                .parse::<u32>()
            else {
                continue;
            };
            let index = file_name.trim_end_matches("_input");
            fans.push(FanInfo {
                name: format!("{chip_name} {index}"),
                speed_rpm: Some(rpm),
            });
        }
    }

    fans
}

/// Repli si aucun `fanN_input` n'est exposé (cas fréquent sur laptop) :
/// `/sys/class/thermal/cooling_device*` (world-readable) donne au moins la
/// présence et un niveau grossier, pas de régime précis.
fn cooling_device_fans() -> Vec<FanInfo> {
    let mut fans = Vec::new();
    let Ok(entries) = fs::read_dir("/sys/class/thermal") else {
        return fans;
    };

    let mut devices: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().starts_with("cooling_device"))
        .collect();
    devices.sort_by_key(|e| e.file_name());

    let mut fan_index = 0;
    for device in &devices {
        let kind = fs::read_to_string(device.path().join("type"))
            .map(|s| s.trim().to_string())
            .unwrap_or_default();
        if kind != "Fan" {
            continue;
        }
        let cur_state = fs::read_to_string(device.path().join("cur_state"))
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "?".to_string());
        let max_state = fs::read_to_string(device.path().join("max_state"))
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "?".to_string());

        fans.push(FanInfo {
            name: format!("Ventilateur {fan_index} (niveau {cur_state}/{max_state})"),
            speed_rpm: None,
        });
        fan_index += 1;
    }

    fans
}

pub fn collect() -> Vec<FanInfo> {
    let fans = hwmon_fans();
    if !fans.is_empty() {
        return fans;
    }
    cooling_device_fans()
}
