use super::BluetoothDeviceInfo;
use std::process::Command;

pub fn collect() -> Vec<BluetoothDeviceInfo> {
    let Ok(output) = Command::new("system_profiler")
        .arg("SPBluetoothDataType")
        .output()
    else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(&output.stdout);

    // Les noms d'appareils sont les lignes indentées se terminant par ':'
    // sans valeur, sous la sous-section des appareils connectés/appairés.
    text.lines()
        .filter(|line| {
            line.starts_with("          ")
                && !line.starts_with("            ")
                && line.trim_end().ends_with(':')
        })
        .map(|line| BluetoothDeviceInfo {
            name: line.trim().trim_end_matches(':').to_string(),
        })
        .collect()
}
