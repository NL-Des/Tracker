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
    let output = std::process::Command::new("smartctl")
        .args(["-H", &device_path])
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&output.stdout);
    text.lines().find_map(|line| {
        let line = line.trim();
        line.strip_prefix("SMART overall-health self-assessment test result:")
            .map(|v| v.trim().to_string())
    })
}

fn map_disk(disk: &Disk) -> DiskInfo {
    let total_gb = disk.total_space() / 1024 / 1024 / 1024;
    let available_gb = disk.available_space() / 1024 / 1024 / 1024;
    let name = disk.name().to_string_lossy().to_string();
    let smart_health = if name.contains("nvme") {
        read_smart_health(&name)
    } else {
        None
    };
    DiskInfo {
        name,
        kind: disk.kind().to_string(),
        file_system: disk.file_system().to_string_lossy().to_string(),
        mount_point: disk.mount_point().to_string_lossy().to_string(),
        is_removable: disk.is_removable(),
        total_gb,
        used_gb: total_gb - available_gb,
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
