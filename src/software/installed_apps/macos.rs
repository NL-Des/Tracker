use super::InstalledAppInfo;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn applications_dirs() -> Vec<PathBuf> {
    let mut dirs = vec![PathBuf::from("/Applications")];
    if let Ok(home) = std::env::var("HOME") {
        dirs.push(PathBuf::from(home).join("Applications"));
    }
    dirs
}

fn plist_value(bundle_path: &Path, key: &str) -> Option<String> {
    let plist_path = bundle_path.join("Contents/Info.plist");
    let output = Command::new("plutil")
        .args(["-extract", key, "raw", "-o", "-"])
        .arg(&plist_path)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if text.is_empty() {
        None
    } else {
        Some(text)
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
