use super::InstalledAppInfo;
use std::fs;
use std::path::{Path, PathBuf};

fn applications_dirs() -> Vec<PathBuf> {
    let mut dirs = vec![PathBuf::from("/Applications")];
    if let Ok(home) = std::env::var("HOME") {
        dirs.push(PathBuf::from(home).join("Applications"));
    }
    dirs
}

fn plist_value(bundle_path: &Path, key: &str) -> Option<String> {
    let plist_path = bundle_path.join("Contents/Info.plist").to_string_lossy().into_owned();
    let text = crate::command::run("plutil", &["-extract", key, "raw", "-o", "-", &plist_path])?;
    let text = text.trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}

pub fn collect() -> Vec<InstalledAppInfo> {
    let mut apps = Vec::new();

    for dir in applications_dirs() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("app") {
                continue;
            }
            let name = plist_value(&path, "CFBundleName")
                .or_else(|| path.file_stem().map(|s| s.to_string_lossy().to_string()));
            let Some(name) = name else {
                continue;
            };
            let version = plist_value(&path, "CFBundleShortVersionString");
            apps.push(InstalledAppInfo {
                name,
                version,
                publisher: None,
                source: "app-bundle".to_string(),
            });
        }
    }

    apps
}
