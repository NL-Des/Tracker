use std::path::PathBuf;
use tracker::consent::{ConsentConfig, ConsentPreset, HardwareConsent, SoftwareConsent};
use tracker::remote_export::RemoteExportConfig;
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
pub fn get_remote_export_config() -> Result<RemoteExportConfig, String> {
    tracker::remote_export::load()
        .map_err(|e| e.to_string())
        .map(|opt| opt.unwrap_or_default())
}

#[tauri::command]
pub fn save_remote_export_config(config: RemoteExportConfig) -> Result<(), String> {
    tracker::remote_export::save(&config).map_err(|e| e.to_string())
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

        // L'historique local stocke toujours le rapport complet, non filtré :
        // le filtrage "np" ne s'applique qu'aux exports fichiers ci-dessous.
        if let Err(e) = tracker::storage::record_snapshot(&report) {
            eprintln!("Erreur lors de l'enregistrement en base : {e}");
        }

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

        // Étape supplémentaire best-effort, symétrique à l'enregistrement
        // SQLite ci-dessus : ne doit jamais faire échouer `collect_and_export`,
        // même si la config est corrompue ou le réseau indisponible. On
        // avale volontairement toute erreur de lecture de
        // `remote_export.json` : une config distante illisible ne doit pas
        // bloquer les exports fichiers déjà écrits.
        let remote_config = tracker::remote_export::load().ok().flatten().unwrap_or_default();
        if remote_config.enabled {
            match report.to_json_pretty_filtered(&consent) {
                Ok(json_body) => {
                    if let Err(e) = tracker::remote_export::send_report(&remote_config, &json_body) {
                        eprintln!("Erreur lors de l'envoi du rapport au serveur distant : {e}");
                    }
                }
                Err(e) => eprintln!(
                    "Erreur lors de la sérialisation du rapport pour l'export distant : {e}"
                ),
            }
        }

        Ok(written)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub fn list_snapshots() -> Result<Vec<tracker::storage::SnapshotSummary>, String> {
    let conn = tracker::storage::open()?;
    tracker::storage::list_snapshots(&conn)
}

#[tauri::command]
pub fn get_snapshot(id: i64) -> Result<Option<String>, String> {
    let conn = tracker::storage::open()?;
    tracker::storage::get_snapshot_json(&conn, id)
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
