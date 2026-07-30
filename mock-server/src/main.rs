use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

struct AppState {
    expected_token: Option<String>,
    reports_dir: PathBuf,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let expected_token = std::env::var("MOCK_SERVER_TOKEN")
        .ok()
        .filter(|t| !t.is_empty());
    let reports_dir = std::env::var("MOCK_SERVER_DATA_DIR")
        .unwrap_or_else(|_| "/data/reports".to_string())
        .into();
    tokio::fs::create_dir_all(&reports_dir)
        .await
        .expect("impossible de créer le répertoire de stockage des rapports");

    if expected_token.is_none() {
        tracing::warn!("MOCK_SERVER_TOKEN non défini : aucune vérification d'authentification ne sera appliquée");
    }

    let state = Arc::new(AppState {
        expected_token,
        reports_dir,
    });

    let app = Router::new()
        .route("/health", get(health))
        .route("/report", post(receive_report))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("mock-server à l'écoute sur {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn receive_report(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: Bytes,
) -> (StatusCode, Json<serde_json::Value>) {
    if let Some(expected) = &state.expected_token {
        let provided = headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "));
        if provided != Some(expected.as_str()) {
            tracing::warn!("requête rejetée : token d'authentification absent ou invalide");
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "status": "unauthorized" })),
            );
        }
    }

    let received_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    tracing::info!(bytes = body.len(), received_at, "rapport reçu");

    let file_path = state
        .reports_dir
        .join(format!("{received_at}.json"));
    if let Err(e) = tokio::fs::write(&file_path, &body).await {
        tracing::error!("échec d'écriture du rapport sur disque : {e}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "status": "storage_error" })),
        );
    }

    (
        StatusCode::OK,
        Json(serde_json::json!({ "status": "received" })),
    )
}
