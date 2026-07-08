use super::BluetoothDeviceInfo;
use std::process::Command;

/// Format de sortie de `bluetoothctl devices Paired` :
/// `Device XX:XX:XX:XX:XX:XX Nom de l'appareil`
pub fn collect() -> Vec<BluetoothDeviceInfo> {
    let Ok(output) = Command::new("bluetoothctl")
        .args(["devices", "Paired"])
        .output()
    else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(&output.stdout);

    text.lines()
        .filter_map(|line| {
            let mut parts = line.splitn(3, ' ');
            parts.next().filter(|word| *word == "Device")?;
            parts.next()?; // adresse MAC
            let name = parts.next()?.trim();
            (!name.is_empty()).then(|| BluetoothDeviceInfo {
                name: name.to_string(),
            })
        })
        .collect()
}
