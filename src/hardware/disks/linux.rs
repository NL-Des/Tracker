use std::fs;

/// Modèle/numéro de série via `/sys/block/<device>/device/{model,serial}`
/// (lecture libre, pas de root requis). `device_name` est le nom sysinfo du
/// disque (ex. `/dev/sda` ou `sda`) : on ne garde que le nom de base et on
/// retire un éventuel suffixe de partition (`sda1` -> `sda`, `nvme0n1p1` ->
/// `nvme0n1`) pour retrouver l'entrée `/sys/block` correspondante.
pub fn read(device_name: &str) -> (Option<String>, Option<String>) {
    let base_name = device_name.rsplit('/').next().unwrap_or(device_name);
    let block_name = strip_partition_suffix(base_name);
    let base = format!("/sys/block/{block_name}/device");

    let model = fs::read_to_string(format!("{base}/model"))
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    let serial = fs::read_to_string(format!("{base}/serial"))
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    (model, serial)
}

/// Retire un suffixe de partition d'un nom de device Linux, en s'appuyant sur
/// l'existence de l'entrée dans `/sys/block` plutôt que sur une heuristique
/// de nommage pure : `nvme0n1` (disque entier) et `sda1`/`nvme0n1p1`
/// (partitions) suivent des conventions différentes (avec ou sans séparateur
/// `p`), donc deviner par regex serait fragile.
fn strip_partition_suffix(name: &str) -> String {
    if std::path::Path::new(&format!("/sys/block/{name}")).exists() {
        return name.to_string();
    }
    // Partitions NVMe/eMMC : "nvme0n1p1" -> "nvme0n1", "mmcblk0p1" -> "mmcblk0".
    if let Some(p_pos) = name.rfind('p') {
        let before = &name[..p_pos];
        if before.ends_with(|c: char| c.is_ascii_digit())
            && std::path::Path::new(&format!("/sys/block/{before}")).exists()
        {
            return before.to_string();
        }
    }
    // Partitions SCSI/SATA/virtio : "sda1" -> "sda".
    let trimmed = name.trim_end_matches(|c: char| c.is_ascii_digit());
    if !trimmed.is_empty() && trimmed != name {
        return trimmed.to_string();
    }
    name.to_string()
}
