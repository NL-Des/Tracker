#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct ProxyConfigInfo {
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
    pub no_proxy: Option<String>,
    pub source: String,
}

fn env_var_ci(key: &str) -> Option<String> {
    std::env::var(key).ok().or_else(|| std::env::var(key.to_lowercase()).ok())
}

fn from_env() -> Option<ProxyConfigInfo> {
    let http_proxy = env_var_ci("HTTP_PROXY");
    let https_proxy = env_var_ci("HTTPS_PROXY");
    let no_proxy = env_var_ci("NO_PROXY");
    if http_proxy.is_none() && https_proxy.is_none() {
        return None;
    }
    Some(ProxyConfigInfo {
        http_proxy,
        https_proxy,
        no_proxy,
        source: "env".to_string(),
    })
}

/// Vérifie d'abord les variables d'environnement standard (indépendantes de
/// l'OS), puis retombe sur la configuration système native par plateforme.
/// Infaillible par design : aucune configuration détectée renvoie `None`.
pub fn collect() -> Option<ProxyConfigInfo> {
    from_env().or_else(|| crate::os_dispatch::dispatch_os!(linux::collect(), macos::collect(), windows::collect(), None))
}
