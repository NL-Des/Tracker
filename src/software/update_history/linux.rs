use super::{UpdateHistoryEntryInfo, MAX_ENTRIES};
use std::fs;

/// `/var/log/apt/history.log` est généralement lisible sans root (droits
/// d'un fichier de log classique) ; renvoie les entrées les plus récentes
/// en tête de liste.
pub fn collect() -> Vec<UpdateHistoryEntryInfo> {
    let Ok(contents) = fs::read_to_string("/var/log/apt/history.log") else {
        return Vec::new();
    };

    let mut entries = Vec::new();
    let mut current_date: Option<String> = None;
    let mut current_command: Option<String> = None;

    for line in contents.lines() {
        if let Some(date) = line.strip_prefix("Start-Date:") {
            current_date = Some(date.trim().to_string());
        } else if let Some(command) = line.strip_prefix("Commandline:") {
            current_command = Some(command.trim().to_string());
        } else if line.starts_with("End-Date:") {
            if let (Some(date), Some(command)) = (current_date.take(), current_command.take()) {
                entries.push(UpdateHistoryEntryInfo {
                    date,
                    description: command,
                });
            }
        }
    }

    entries.reverse();
    entries.truncate(MAX_ENTRIES);
    entries
}
