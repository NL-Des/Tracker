use std::path::PathBuf;
use tracker::consent::ConsentConfig;
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
pub fn save_consent(config: ConsentConfig) -> Result<(), String> {
    tracker::consent::save(&config).map_err(|e| e.to_string())
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
        let dir = PathBuf::from(output_dir);
        let mut written = Vec::new();

        for format in &formats {
            let (path, result): (PathBuf, std::io::Result<()>) = match format.as_str() {
                "json" => {
                    let path = dir.join("tracker_report.json");
                    (path.clone(), report.save_json(&path))
                }
                "markdown" => {
                    let path = dir.join("tracker_report.md");
                    (path.clone(), report.save_markdown(&path))
                }
                "xml" => {
                    let path = dir.join("tracker_report.xml");
                    (path.clone(), report.save_xml(&path))
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
