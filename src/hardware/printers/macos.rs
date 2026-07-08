use super::PrinterInfo;
use std::process::Command;

/// CUPS est aussi le système d'impression standard sur macOS : même
/// commande et même format de sortie que sur Linux. Pas d'équivalent simple
/// en ligne de commande pour les scanners (ImageCaptureCore n'est pas
/// exposé sans dépendance supplémentaire).
pub fn collect() -> Vec<PrinterInfo> {
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
