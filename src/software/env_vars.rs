use serde::Serialize;

const SENSITIVE_KEY_MARKERS: &[&str] = &[
    "TOKEN",
    "SECRET",
    "KEY",
    "PASSWORD",
    "PWD",
    "CREDENTIAL",
    "AUTH",
];

const REDACTED_PLACEHOLDER: &str = "***REDACTED***";

#[derive(Serialize)]
pub struct EnvVarInfo {
    pub key: String,
    pub value: String,
}

fn is_sensitive_key(key: &str) -> bool {
    let upper = key.to_uppercase();
    SENSITIVE_KEY_MARKERS.iter().any(|marker| upper.contains(marker))
}

/// Les variables d'environnement contiennent souvent des secrets (tokens,
/// clés API...). Comme ce rapport est exporté en JSON sur disque, les clés
/// jugées sensibles sont rédigées par défaut.
pub fn collect() -> Vec<EnvVarInfo> {
    std::env::vars()
        .map(|(key, value)| {
            let value = if is_sensitive_key(&key) {
                REDACTED_PLACEHOLDER.to_string()
            } else {
                value
            };
            EnvVarInfo { key, value }
        })
        .collect()
}
