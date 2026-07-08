use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
pub struct DesktopEnvironmentInfo {
    pub desktop: Option<String>,
    pub session_type: Option<String>,
    pub locale: Option<String>,
    pub timezone: Option<String>,
}

fn env_var(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|v| !v.is_empty())
}

#[cfg(target_os = "linux")]
fn read_timezone() -> Option<String> {
    let output = Command::new("timedatectl").output().ok()?;
    let text = String::from_utf8_lossy(&output.stdout);
    text.lines().find_map(|line| {
        line.trim()
            .strip_prefix("Time zone:")
            .map(|v| v.split_whitespace().next().unwrap_or("").to_string())
    })
}

#[cfg(not(target_os = "linux"))]
fn read_timezone() -> Option<String> {
    None
}

/// Infaillible par design : variables d'environnement/commande absentes
/// laissent simplement les champs à `None`.
pub fn collect() -> DesktopEnvironmentInfo {
    DesktopEnvironmentInfo {
        desktop: env_var("XDG_CURRENT_DESKTOP"),
        session_type: env_var("XDG_SESSION_TYPE"),
        locale: env_var("LANG"),
        timezone: read_timezone(),
    }
}
