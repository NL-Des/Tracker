use super::PrinterInfo;
use std::process::Command;

/// `lpstat -p` (CUPS) affiche une ligne par imprimante :
/// `printer <nom> is idle.  enabled since ...`
fn printers() -> Vec<PrinterInfo> {
    let Ok(output) = Command::new("lpstat").arg("-p").output() else {
        return Vec::new();
    };

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.strip_prefix("printer "))
        .filter_map(|rest| rest.split_whitespace().next())
        .map(|name| PrinterInfo {
            name: name.to_string(),
            kind: "Imprimante".to_string(),
        })
        .collect()
}

/// `scanimage -L` (SANE) affiche une ligne par scanner :
/// `device `URI' is a Fabricant Modèle flatbed scanner`
fn scanners() -> Vec<PrinterInfo> {
    let Ok(output) = Command::new("scanimage").arg("-L").output() else {
        return Vec::new();
    };

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.strip_prefix("device "))
        .filter_map(|rest| rest.split_once("is a "))
        .map(|(_, name)| PrinterInfo {
            name: name.trim().to_string(),
            kind: "Scanner".to_string(),
        })
        .collect()
}

pub fn collect() -> Vec<PrinterInfo> {
    let mut devices = printers();
    devices.extend(scanners());
    devices
}
