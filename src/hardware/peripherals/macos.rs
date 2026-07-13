use super::PeripheralInfo;

/// Le clavier interne d'un Mac n'apparaît pas de façon fiable dans
/// `system_profiler` (pas de catégorie USB/Bluetooth dédiée) : il n'est
/// volontairement pas recherché ici plutôt que de renvoyer une donnée
/// approximative.
fn speakers() -> Vec<PeripheralInfo> {
    let Some(text) = crate::command::run("system_profiler", &["SPAudioDataType"]) else {
        return Vec::new();
    };

    text.lines()
        .filter(|line| {
            line.starts_with("    ")
                && !line.starts_with("      ")
                && line.trim_end().ends_with(':')
        })
        .map(|line| PeripheralInfo {
            name: line.trim().trim_end_matches(':').to_string(),
            kind: "Enceintes".to_string(),
        })
        .collect()
}

pub fn collect() -> Vec<PeripheralInfo> {
    speakers()
}
