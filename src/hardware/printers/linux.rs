use super::PrinterInfo;

/// `lpstat -p` (CUPS) affiche une ligne par imprimante :
/// `printer <nom> is idle.  enabled since ...`
/// Statut de sortie non vérifié ici (comportement existant conservé).
fn printers() -> Vec<PrinterInfo> {
    let Some(text) = crate::command::run_lenient("lpstat", &["-p"]) else {
        return Vec::new();
    };

    text.lines()
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
/// Statut de sortie non vérifié ici (comportement existant conservé).
fn scanners() -> Vec<PrinterInfo> {
    let Some(text) = crate::command::run_lenient("scanimage", &["-L"]) else {
        return Vec::new();
    };

    text.lines()
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
