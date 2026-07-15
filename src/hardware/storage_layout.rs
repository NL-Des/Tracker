#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct PartitionInfo {
    pub device: String,
    pub fs_type: String,
    pub size_gb: u64,
}

#[derive(Serialize)]
pub struct LvmVolumeInfo {
    pub vg_name: String,
    pub lv_name: String,
    pub size_gb: u64,
}

#[derive(Serialize)]
pub struct RaidArrayInfo {
    pub device: String,
    pub level: String,
    pub state: String,
    pub devices: Vec<String>,
}

#[derive(Serialize)]
pub struct StorageLayoutInfo {
    pub partitions: Vec<PartitionInfo>,
    pub lvm_volumes: Vec<LvmVolumeInfo>,
    pub raid_arrays: Vec<RaidArrayInfo>,
}

/// Infaillible par design : absence d'outil (`lsblk`/`lvs`/`diskutil`...) ou
/// erreur d'accès renvoient des `Vec` vides. Complète les points de montage
/// déjà collectés par `disks.rs` avec la table de partitions, les volumes
/// LVM et les tableaux RAID logiciels.
pub fn collect() -> StorageLayoutInfo {
    crate::os_dispatch::dispatch_os!(
        linux::collect(),
        macos::collect(),
        windows::collect(),
        StorageLayoutInfo { partitions: Vec::new(), lvm_volumes: Vec::new(), raid_arrays: Vec::new() }
    )
}
