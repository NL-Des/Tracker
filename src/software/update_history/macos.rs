use super::{UpdateHistoryEntryInfo, MAX_ENTRIES};
use std::process::Command;

/// `softwareupdate --history` liste l'historique des mises à jour Apple
/// installées, en lecture seule sans droits admin.
pub fn collect() -> Vec<UpdateHistoryEntryInfo> {
    let Ok(output) = Command::new("softwareupdate").arg("--history").output() else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(&output.stdout);

    // Format tabulaire : "Display Name    Version    Date, Time" — pas de
    // séparateur fiable pour isoler la date, on garde la ligne entière.
    let mut entries: Vec<UpdateHistoryEntryInfo> = text
        .lines()
        .skip(2)
        .filter(|line| !line.trim().is_empty())
        .map(|line| UpdateHistoryEntryInfo {
            date: "?".to_string(),
            description: line.trim().to_string(),
        })
        .collect();

    entries.truncate(MAX_ENTRIES);
    entries
}
