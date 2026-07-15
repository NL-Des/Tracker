#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;
mod common;

use crate::software::docker::{DockerImageInfo, DockerVolumeInfo};

/// Le CLI `podman` a une surface quasi identique à `docker` (mêmes
/// sous-commandes/flags JSON), donc les structs `DockerImageInfo`/
/// `DockerVolumeInfo` sont réutilisées telles quelles.
/// Infaillible par design : absence du CLI `podman` ou erreur d'accès
/// renvoient simplement un `Vec` vide. Lecture seule, aucune élévation
/// requise.
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
