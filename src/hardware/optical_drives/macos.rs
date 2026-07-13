use super::OpticalDriveInfo;

/// macOS ne fabrique plus de lecteur de disquette depuis longtemps : seuls
/// les lecteurs optiques (externes, USB) sont donc recherchés ici.
pub fn collect() -> Vec<OpticalDriveInfo> {
    let Some(text) = crate::command::run("system_profiler", &["SPDiscBurningDataType"]) else {
        return Vec::new();
    };

    // Les noms de lecteurs sont les lignes indentées de 4 espaces se
    // terminant par ':' sans valeur (contrairement aux lignes de propriétés,
    // indentées de 6 espaces et contenant "clé: valeur").
    text.lines()
        .filter(|line| {
            line.starts_with("    ")
                && !line.starts_with("      ")
                && line.trim_end().ends_with(':')
        })
        .map(|line| OpticalDriveInfo {
            name: line.trim().trim_end_matches(':').to_string(),
            vendor: None,
            kind: "CD/DVD".to_string(),
        })
        .collect()
}
