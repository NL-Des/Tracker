use super::InstalledAppInfo;
use std::fs;
use std::path::PathBuf;

fn desktop_dirs() -> Vec<PathBuf> {
    let mut dirs = vec![PathBuf::from("/usr/share/applications")];
    if let Ok(home) = std::env::var("HOME") {
        dirs.push(PathBuf::from(home).join(".local/share/applications"));
    }
    dirs
}

/// Parse un fichier `.desktop` minimal : ne lit que la section
/// `[Desktop Entry]` et s'arrête à la section suivante.
fn parse_desktop_file(contents: &str) -> Option<InstalledAppInfo> {
    let mut name = None;
    let mut version = None;
    let mut in_desktop_entry = false;
    let mut is_application = true;

    for line in contents.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_desktop_entry = line == "[Desktop Entry]";
            continue;
        }
        if !in_desktop_entry {
            continue;
        }
        if let Some(value) = line.strip_prefix("Name=") {
            if name.is_none() {
                name = Some(value.to_string());
            }
        } else if let Some(value) = line.strip_prefix("Version=") {
            version = Some(value.to_string());
        } else if let Some(value) = line.strip_prefix("Type=") {
            is_application = value == "Application";
        } else if line.starts_with("NoDisplay=true") {
            is_application = false;
        }
    }

    if !is_application {
        return None;
    }

    name.map(|name| InstalledAppInfo {
        name,
        version,
        publisher: None,
        source: "desktop-file".to_string(),
    })
}

pub fn collect() -> Vec<InstalledAppInfo> {
    let mut apps = Vec::new();
    let mut seen_names = std::collections::HashSet::new();

    for dir in desktop_dirs() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("desktop") {
                continue;
            }
            let Ok(contents) = fs::read_to_string(&path) else {
                continue;
            };
            if let Some(app) = parse_desktop_file(&contents)
                && seen_names.insert(app.name.clone())
            {
                apps.push(app);
            }
        }
    }

    apps
}
