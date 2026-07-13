#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct ScheduledTaskInfo {
    pub name: String,
    pub schedule: String,
}

/// Infaillible par design : uniquement les tâches de l'utilisateur courant
/// (pas celles de root/autres comptes), aucune élévation requise.
pub fn collect() -> Vec<ScheduledTaskInfo> {
    crate::os_dispatch::dispatch_os!(linux::collect(), macos::collect(), windows::collect(), Vec::new())
}
