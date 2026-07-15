use super::edid::{parse_edid, EdidIds};
use std::fs;

/// Lit `/sys/class/drm/*/edid` (lecture libre, pas de root requis) pour
/// chaque connecteur actif et en extrait les identifiants EDID. L'ordre des
/// entrées n'est pas garanti correspondre exactement à l'ordre des écrans de
/// la crate `display-info` : association best-effort par index.
pub fn read_all() -> Vec<EdidIds> {
    let Ok(entries) = fs::read_dir("/sys/class/drm") else {
        return Vec::new();
    };

    let mut results = Vec::new();
    let mut paths: Vec<_> = entries.filter_map(|e| e.ok()).map(|e| e.path()).collect();
    paths.sort();

    for path in paths {
        let edid_path = path.join("edid");
        let Ok(bytes) = fs::read(&edid_path) else {
            continue;
        };
        if let Some(ids) = parse_edid(&bytes) {
            results.push(ids);
        }
    }

    results
}
