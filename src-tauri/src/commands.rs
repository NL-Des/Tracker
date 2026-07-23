use std::path::PathBuf;
use tracker::consent::{ConsentConfig, ConsentPreset, HardwareConsent, SoftwareConsent};
use tracker::SystemReport;

#[tauri::command]
pub fn set_locale(locale: String) {
    rust_i18n::set_locale(&locale);
}

#[tauri::command]
pub fn get_consent() -> Result<ConsentConfig, String> {
    tracker::consent::load()
        .map_err(|e| e.to_string())
        .map(|opt| opt.unwrap_or_default())
}

#[tauri::command]
pub fn save_consent(mut config: ConsentConfig) -> Result<(), String> {
    config.accepted_at_unix = Some(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_secs(),
    );
    tracker::consent::save(&config).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_hardware_fields() -> Result<Vec<String>, String> {
    let value = serde_json::to_value(HardwareConsent::default()).map_err(|e| e.to_string())?;
    let object = value
        .as_object()
        .ok_or("HardwareConsent ne sérialise pas en objet")?;
    Ok(object.keys().cloned().collect())
}

#[tauri::command]
pub fn list_software_fields() -> Result<Vec<String>, String> {
    let value = serde_json::to_value(SoftwareConsent::default()).map_err(|e| e.to_string())?;
    let object = value
        .as_object()
        .ok_or("SoftwareConsent ne sérialise pas en objet")?;
    Ok(object.keys().cloned().collect())
}

#[tauri::command]
pub fn get_preset(name: String) -> Result<ConsentConfig, String> {
    let preset = match name.as_str() {
        "none" => ConsentPreset::None,
        "minimum" => ConsentPreset::Minimum,
        "medium" => ConsentPreset::Medium,
        "maximum" => ConsentPreset::Maximum,
        other => return Err(format!("preset inconnu : {other}")),
    };
    Ok(preset.to_config())
}

#[tauri::command]
pub async fn collect_and_export(
    formats: Vec<String>,
    output_dir: String,
) -> Result<Vec<String>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        // `SystemReport::collect()` dort volontairement (échantillonnage CPU) —
        // toujours l'exécuter hors du thread de l'event loop de la webview.
        let report = SystemReport::collect();
        // Le filtrage "np" (étape 9) n'existe que côté GUI : on charge le
        // consentement courant de l'utilisateur avant l'export.
        let consent = tracker::consent::load()
            .map_err(|e| e.to_string())?
            .unwrap_or_default();
        let dir = PathBuf::from(output_dir);
        let mut written = Vec::new();

        for format in &formats {
            let (path, result): (PathBuf, std::io::Result<()>) = match format.as_str() {
                "json" => {
                    let path = dir.join("tracker_report.json");
                    (path.clone(), report.save_json_filtered(&path, &consent))
                }
                "markdown" => {
                    let path = dir.join("tracker_report.md");
                    (path.clone(), report.save_markdown_filtered(&path, &consent))
                }
                "xml" => {
                    let path = dir.join("tracker_report.xml");
                    (path.clone(), report.save_xml_filtered(&path, &consent))
                }
                other => {
                    return Err(format!("format d'export inconnu : {other}"));
                }
            };
            result.map_err(|e| e.to_string())?;
            written.push(path.display().to_string());
        }

        Ok(written)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_hardware_fields_matches_hardware_fields_constant() {
        let mut got = list_hardware_fields().unwrap();
        got.sort();
        let mut expected: Vec<String> = tracker::hardware::HARDWARE_FIELDS
            .iter()
            .map(|s| s.to_string())
            .collect();
        expected.sort();
        assert_eq!(got, expected);
    }

    #[test]
    fn list_software_fields_matches_software_fields_constant() {
        let mut got = list_software_fields().unwrap();
        got.sort();
        let mut expected: Vec<String> = tracker::software::SOFTWARE_FIELDS
            .iter()
            .map(|s| s.to_string())
            .collect();
        expected.sort();
        assert_eq!(got, expected);
    }
}
