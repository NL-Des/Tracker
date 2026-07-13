use serde::Serialize;

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

/// Statut de sortie non vérifié ici (comportement existant conservé).
#[cfg(target_os = "linux")]
fn read_timezone_linux() -> Option<String> {
    let text = crate::command::run_lenient("timedatectl", &[])?;
    text.lines().find_map(|line| {
        line.trim()
            .strip_prefix("Time zone:")
            .map(|v| v.split_whitespace().next().unwrap_or("").to_string())
    })
}

fn read_timezone() -> Option<String> {
    crate::os_dispatch::dispatch_os!(read_timezone_linux(), None, None, None)
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
