use super::PrinterInfo;

/// CUPS est aussi le système d'impression standard sur macOS : même
/// commande et même format de sortie que sur Linux. Pas d'équivalent simple
/// en ligne de commande pour les scanners (ImageCaptureCore n'est pas
/// exposé sans dépendance supplémentaire).
/// Statut de sortie non vérifié ici (comportement existant conservé).
pub fn collect() -> Vec<PrinterInfo> {
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
