use super::{UpdateHistoryEntryInfo, MAX_ENTRIES};

/// `softwareupdate --history` liste l'historique des mises à jour Apple
/// installées, en lecture seule sans droits admin.
pub fn collect() -> Vec<UpdateHistoryEntryInfo> {
    let Some(text) = crate::command::run("softwareupdate", &["--history"]) else {
        return Vec::new();
    };

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
