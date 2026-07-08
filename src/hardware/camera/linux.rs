use super::CameraInfo;
use std::fs;

/// Un capteur physique expose souvent plusieurs nœuds `videoN` (capture +
/// métadonnées) partageant le même nom : on ne garde que la première
/// occurrence de chaque nom pour éviter les doublons.
pub fn collect() -> Vec<CameraInfo> {
    let mut cameras = Vec::new();
    let Ok(mut entries) = fs::read_dir("/sys/class/video4linux")
        .map(|entries| entries.filter_map(|e| e.ok()).collect::<Vec<_>>())
    else {
        return cameras;
    };
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let Ok(name) = fs::read_to_string(entry.path().join("name")) else {
            continue;
        };
        let name = name.trim().to_string();
        if name.is_empty() || cameras.iter().any(|c: &CameraInfo| c.name == name) {
            continue;
        }
        cameras.push(CameraInfo { name });
    }

    cameras
}
