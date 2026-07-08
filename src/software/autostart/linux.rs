use super::AutostartEntryInfo;
use std::fs;

/// `~/.config/autostart/*.desktop` (spécification XDG), lecture libre.
pub fn collect() -> Vec<AutostartEntryInfo> {
    let Ok(home) = std::env::var("HOME") else {
        return Vec::new();
    };
    let dir = std::path::PathBuf::from(home).join(".config/autostart");
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };

    entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|ext| ext.to_str()) == Some("desktop"))
        .filter_map(|entry| {
            let contents = fs::read_to_string(entry.path()).ok()?;
            let mut name = None;
            let mut command = None;
            let mut enabled = true;
            for line in contents.lines() {
                if let Some(v) = line.strip_prefix("Name=") {
                    if name.is_none() {
                        name = Some(v.to_string());
                    }
                } else if let Some(v) = line.strip_prefix("Exec=") {
                    command = Some(v.to_string());
                } else if line.trim() == "Hidden=true" || line.trim() == "X-GNOME-Autostart-enabled=false" {
                    enabled = false;
                }
            }
            if !enabled {
                return None;
            }
            name.map(|name| AutostartEntryInfo { name, command })
        })
        .collect()
}
