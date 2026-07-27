use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::Duration;

const REMOTE_EXPORT_FILE_NAME: &str = "remote_export.json";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

/// Config de l'export HTTP automatique déclenché après chaque collecte.
/// `auth_token` anticipe une auth Bearer future côté serveur externe, même
/// si aucun serveur réel n'existe encore et que ce champ n'est pas exposé
/// dans l'UI v1 : le coût d'ajout est nul et évite une migration de schéma
/// plus tard.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct RemoteExportConfig {
    pub enabled: bool,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
}

fn config_path_in(base_dir: &Path) -> PathBuf {
    base_dir.join(REMOTE_EXPORT_FILE_NAME)
}

pub fn config_path() -> std::io::Result<PathBuf> {
    Ok(config_path_in(&crate::consent::config_dir()?))
}

fn load_from(path: &Path) -> std::io::Result<Option<RemoteExportConfig>> {
    if !path.exists() {
        return Ok(None);
    }
    let content = std::fs::read_to_string(path)?;
    let config = serde_json::from_str(&content)?;
    Ok(Some(config))
}

fn save_to(path: &Path, config: &RemoteExportConfig) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(config)?;
    std::fs::write(path, content)
}

pub fn load() -> std::io::Result<Option<RemoteExportConfig>> {
    load_from(&config_path()?)
}

pub fn save(config: &RemoteExportConfig) -> std::io::Result<()> {
    save_to(&config_path()?, config)
}

/// Envoie `json_body` en POST vers `config.url`. No-op si `enabled` est
/// faux. Une seule tentative, aucun retry : le serveur cible n'existe pas
/// encore et un mécanisme de retry/backoff ne pourrait pas être validé
/// contre un vrai comportement serveur — on garde ça simple pour la v1 et
/// on laisse l'appelant logguer l'échec sans jamais faire échouer la
/// collecte (même philosophie que `storage::record_snapshot`).
pub fn send_report(config: &RemoteExportConfig, json_body: &str) -> Result<(), String> {
    if !config.enabled {
        return Ok(());
    }
    if config.url.trim().is_empty() {
        return Err("URL d'export distant vide".to_string());
    }

    let client = reqwest::blocking::Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .build()
        .map_err(|e| e.to_string())?;

    let mut request = client
        .post(&config.url)
        .header("Content-Type", "application/json")
        .body(json_body.to_string());

    if let Some(token) = config.auth_token.as_deref().filter(|t| !t.is_empty()) {
        request = request.bearer_auth(token);
    }

    let response = request.send().map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err(format!("le serveur distant a répondu {}", response.status()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader, Read, Write};
    use std::net::TcpListener;

    #[test]
    fn default_config_is_disabled_with_empty_url() {
        let config = RemoteExportConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.url, "");
    }

    #[test]
    fn save_and_load_round_trip_via_temp_dir() {
        let dir = std::env::temp_dir()
            .join(format!("tracker-remote-export-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = config_path_in(&dir);

        let config = RemoteExportConfig {
            enabled: true,
            url: "https://example.invalid/reports".to_string(),
            auth_token: Some("secret".to_string()),
        };
        save_to(&path, &config).unwrap();
        let loaded = load_from(&path).unwrap().expect("le fichier vient d'être créé");
        assert_eq!(loaded, config);

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn send_report_is_noop_when_disabled() {
        let config = RemoteExportConfig {
            enabled: false,
            url: "http://127.0.0.1:9".into(),
            auth_token: None,
        };
        // Port 9 (discard) : si ce n'était pas un no-op, la requête échouerait/traînerait.
        assert!(send_report(&config, "{}").is_ok());
    }

    // Serveur HTTP jetable in-process (TcpListener brut) pour observer la
    // requête réellement envoyée, sans ajouter de dépendance de mock HTTP :
    // on lit la requête ligne par ligne et on répond à la main.
    fn spawn_one_shot_server(status_line: &'static str) -> (String, std::thread::JoinHandle<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut request_line = String::new();
            reader.read_line(&mut request_line).unwrap();

            let mut content_length = 0usize;
            loop {
                let mut header_line = String::new();
                reader.read_line(&mut header_line).unwrap();
                if header_line == "\r\n" || header_line.is_empty() {
                    break;
                }
                if let Some(v) = header_line.to_lowercase().strip_prefix("content-length:") {
                    content_length = v.trim().parse().unwrap_or(0);
                }
            }
            let mut body = vec![0u8; content_length];
            reader.read_exact(&mut body).unwrap();

            stream.write_all(status_line.as_bytes()).unwrap();
            format!("{request_line}{}", String::from_utf8_lossy(&body))
        });
        (format!("http://{addr}"), handle)
    }

    #[test]
    fn send_report_posts_json_body_and_succeeds_on_2xx() {
        let (url, handle) = spawn_one_shot_server("HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n");
        let config = RemoteExportConfig { enabled: true, url, auth_token: None };
        let result = send_report(&config, r#"{"hello":"world"}"#);
        let received = handle.join().unwrap();
        assert!(result.is_ok());
        assert!(received.starts_with("POST "));
        assert!(received.ends_with(r#"{"hello":"world"}"#));
    }

    #[test]
    fn send_report_fails_on_server_error_status() {
        let (url, handle) =
            spawn_one_shot_server("HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n");
        let config = RemoteExportConfig { enabled: true, url, auth_token: None };
        let result = send_report(&config, "{}");
        handle.join().unwrap();
        assert!(result.is_err());
    }
}
