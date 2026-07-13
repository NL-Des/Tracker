use super::PciDeviceInfo;

pub fn collect() -> Vec<PciDeviceInfo> {
    let Some(text) = crate::command::run("system_profiler", &["SPPCIDataType"]) else {
        return Vec::new();
    };

    // Les noms de périphériques sont les lignes indentées de 4 espaces se
    // terminant par ':' sans valeur.
    text.lines()
        .filter(|line| {
            line.starts_with("    ") && !line.starts_with("      ") && line.trim_end().ends_with(':')
        })
        .map(|line| PciDeviceInfo {
            name: line.trim().trim_end_matches(':').to_string(),
            class: "PCI".to_string(),
        })
        .collect()
}
