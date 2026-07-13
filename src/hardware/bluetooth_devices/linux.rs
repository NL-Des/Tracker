use super::BluetoothDeviceInfo;

/// Format de sortie de `bluetoothctl devices Paired` :
/// `Device XX:XX:XX:XX:XX:XX Nom de l'appareil`
pub fn collect() -> Vec<BluetoothDeviceInfo> {
    let Some(text) = crate::command::run("bluetoothctl", &["devices", "Paired"]) else {
        return Vec::new();
    };

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
