use super::CameraInfo;
use std::process::Command;

pub fn collect() -> Vec<CameraInfo> {
    let Ok(output) = Command::new("system_profiler")
        .arg("SPCameraDataType")
        .output()
    else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(&output.stdout);

    // Les noms de caméras sont les lignes indentées de 4 espaces se
    // terminant par ':' sans valeur (contrairement aux lignes de propriétés,
    // indentées de 6 espaces et contenant "clé: valeur").
    text.lines()
        .filter(|line| {
            line.starts_with("    ")
                && !line.starts_with("      ")
                && line.trim_end().ends_with(':')
        })
        .map(|line| CameraInfo {
            name: line.trim().trim_end_matches(':').to_string(),
        })
        .collect()
}
