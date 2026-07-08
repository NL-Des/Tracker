use super::PciDeviceInfo;
use std::process::Command;

pub fn collect() -> Vec<PciDeviceInfo> {
    let Ok(output) = Command::new("system_profiler")
        .arg("SPPCIDataType")
        .output()
    else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(&output.stdout);

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
