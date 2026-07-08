use super::AutostartEntryInfo;
use std::fs;

/// Agents `launchd` de l'utilisateur courant (`~/Library/LaunchAgents`),
/// lecture libre, distincts des daemons système qui nécessitent root.
pub fn collect() -> Vec<AutostartEntryInfo> {
    let Ok(home) = std::env::var("HOME") else {
        return Vec::new();
    };
    let dir = std::path::PathBuf::from(home).join("Library/LaunchAgents");
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };

    entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|ext| ext.to_str()) == Some("plist"))
        .map(|entry| AutostartEntryInfo {
            name: entry
                .path()
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default(),
            command: None,
        })
        .collect()
}
