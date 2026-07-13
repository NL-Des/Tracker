use super::BluetoothDeviceInfo;

pub fn collect() -> Vec<BluetoothDeviceInfo> {
    let Some(text) = crate::command::run("system_profiler", &["SPBluetoothDataType"]) else {
        return Vec::new();
    };

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
