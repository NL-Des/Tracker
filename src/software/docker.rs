#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;
mod common;

use serde::Serialize;

#[derive(Serialize)]
pub struct DockerImageInfo {
    pub repository: String,
    pub tag: String,
    pub image_id: String,
    pub size: String,
    pub created: String,
}

#[derive(Serialize)]
pub struct DockerVolumeInfo {
    pub name: String,
    pub driver: String,
    pub mountpoint: Option<String>,
}

/// Infaillible par design : absence du CLI `docker` ou erreur d'accès
/// renvoient simplement un `Vec` vide. Lecture seule, aucune élévation requise.
pub fn collect_images() -> Vec<DockerImageInfo> {
    crate::os_dispatch::dispatch_os!(
        linux::collect_images(),
        macos::collect_images(),
        windows::collect_images(),
        Vec::new()
    )
}

pub fn collect_volumes() -> Vec<DockerVolumeInfo> {
    crate::os_dispatch::dispatch_os!(
        linux::collect_volumes(),
        macos::collect_volumes(),
        windows::collect_volumes(),
        Vec::new()
    )
}
