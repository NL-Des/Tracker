#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;
use sysinfo::{Disk, Disks};

#[derive(Serialize)]
pub struct DiskInfo {
    pub name: String,
    pub kind: String,
    pub file_system: String,
    pub mount_point: String,
    pub is_removable: bool,
    pub total_gb: u64,
    pub used_gb: u64,
    /// Modèle et numéro de série du disque physique sous-jacent. Lecture
    /// libre sur Linux (sysfs) et Windows (WMI) ; best-effort sur macOS
    /// (`diskutil info` n'expose pas toujours un vrai numéro de série).
    pub model: Option<String>,
    pub serial: Option<String>,
    /// Santé S.M.A.R.T. sommaire ("PASSED"/"FAILED"), via `smartctl -H`.
    /// Souvent lisible sans root sur NVMe (contrairement au SATA/ATA qui
    /// nécessite généralement des privilèges pour les commandes ATA brutes).
    /// `None` si `smartctl` est absent, échoue, ou nécessite une élévation.
    pub smart_health: Option<String>,
}

/// Best effort : `smartctl` n'est pas toujours installé, et beaucoup de
/// disques (surtout SATA) exigent root pour cette commande. Un échec est
/// donc traité comme une simple absence de donnée.
fn read_smart_health(device_name: &str) -> Option<String> {
    let device_path = format!("/dev/{device_name}");
    let text = crate::command::run_lenient("smartctl", &["-H", &device_path])?;
    text.lines().find_map(|line| {
        let line = line.trim();
        line.strip_prefix("SMART overall-health self-assessment test result:")
            .map(|v| v.trim().to_string())
    })
}

/// Modèle/numéro de série du disque physique, via le device (Linux/macOS)
/// ou le point de montage (Windows, ex. `C:\`) selon la plateforme.
#[allow(unused_variables)]
fn read_model_serial(device_name: &str, mount_point: &str) -> (Option<String>, Option<String>) {
    crate::os_dispatch::dispatch_os!(
        linux::read(device_name),
        macos::read(device_name),
        windows::read(mount_point),
        (None, None)
    )
}

fn map_disk(disk: &Disk) -> DiskInfo {
    let total_gb = disk.total_space() / 1024 / 1024 / 1024;
    let available_gb = disk.available_space() / 1024 / 1024 / 1024;
    let name = disk.name().to_string_lossy().to_string();
    let mount_point = disk.mount_point().to_string_lossy().to_string();
    let smart_health = if name.contains("nvme") {
        read_smart_health(&name)
    } else {
        None
    };
    let (model, serial) = read_model_serial(&name, &mount_point);
    DiskInfo {
        name,
        kind: disk.kind().to_string(),
        file_system: disk.file_system().to_string_lossy().to_string(),
        mount_point,
        is_removable: disk.is_removable(),
        total_gb,
        used_gb: total_gb - available_gb,
        model,
        serial,
        smart_health,
    }
}

/// Renvoie (disques physiques, montages virtuels). Les montages virtuels
/// (overlay Docker/containerd, etc.) sont reconnaissables via `DiskKind::Unknown`,
/// contrairement aux disques réels (SSD/HDD) : ce signal déjà fourni par
/// `sysinfo` évite de maintenir une liste de systèmes de fichiers à la main.
pub fn collect() -> (Vec<DiskInfo>, Vec<DiskInfo>) {
    let disks = Disks::new_with_refreshed_list();
    let (virtual_disks, physical_disks): (Vec<DiskInfo>, Vec<DiskInfo>) = disks
        .iter()
        .map(map_disk)
        .partition(|disk| disk.kind == "Unknown");
    (physical_disks, virtual_disks)
}
