use super::GpuInfo;
use std::fs;

fn vendor_name(vendor_id: &str) -> Option<&'static str> {
    match vendor_id.trim().to_lowercase().as_str() {
        "0x10de" => Some("NVIDIA"),
        "0x1002" => Some("AMD"),
        "0x8086" => Some("Intel"),
        "0x1414" => Some("Microsoft"),
        _ => None,
    }
}

struct NvidiaSmiInfo {
    vram_mb: Option<u64>,
    driver_version: Option<String>,
}

/// `nvidia-smi` (driver propriétaire NVIDIA) est en lecture seule et ne
/// nécessite pas root ; absent sur les autres configurations, auquel cas on
/// retombe sur les infos sysfs disponibles (AMD notamment).
fn nvidia_smi_info() -> Option<NvidiaSmiInfo> {
    let text = crate::command::run(
        "nvidia-smi",
        &["--query-gpu=memory.total,driver_version", "--format=csv,noheader,nounits"],
    )?;
    let line = text.lines().next()?;
    let mut parts = line.split(',').map(|s| s.trim());
    Some(NvidiaSmiInfo {
        vram_mb: parts.next().and_then(|s| s.parse::<u64>().ok()),
        driver_version: parts.next().map(|s| s.to_string()),
    })
}

/// Heuristique légère basée sur les IDs vendeur PCI (pas de base pci.ids
/// complète) : suffisant pour identifier les fabricants GPU courants.
pub fn collect() -> Vec<GpuInfo> {
    let mut gpus = Vec::new();
    let Ok(entries) = fs::read_dir("/sys/class/drm") else {
        return gpus;
    };

    // Coûteux (lance un processus) : appelé une seule fois puis réutilisé
    // pour chaque carte détectée en sysfs.
    let nvidia_info = nvidia_smi_info();

    for entry in entries.filter_map(|e| e.ok()) {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        // Ne garder que les cartes principales (cardN), pas les sorties (cardN-DP-1...)
        if !name.starts_with("card") || name.contains('-') {
            continue;
        }

        let device_dir = entry.path().join("device");
        let Some(vendor_id) = fs::read_to_string(device_dir.join("vendor"))
            .ok()
            .map(|s| s.trim().to_string())
        else {
            continue;
        };
        let device_id = fs::read_to_string(device_dir.join("device"))
            .ok()
            .map(|s| s.trim().to_string());

        let vendor = vendor_name(&vendor_id).map(|s| s.to_string());
        let display_name = match (&vendor, &device_id) {
            (Some(v), Some(d)) => format!("{v} (PCI device {d})"),
            (Some(v), None) => v.clone(),
            (None, Some(d)) => format!("Unknown vendor {vendor_id} (PCI device {d})"),
            (None, None) => format!("Unknown vendor {vendor_id}"),
        };

        // VRAM : sysfs `mem_info_vram_total` (AMD, octets) en priorité,
        // sinon `nvidia-smi` si disponible.
        let vram_mb = fs::read_to_string(device_dir.join("mem_info_vram_total"))
            .ok()
            .and_then(|s| s.trim().parse::<u64>().ok())
            .map(|bytes| bytes / 1_000_000)
            .or_else(|| nvidia_info.as_ref().and_then(|i| i.vram_mb));
        let driver_version = nvidia_info.as_ref().and_then(|i| i.driver_version.clone());

        gpus.push(GpuInfo {
            name: display_name,
            vendor,
            vram_mb,
            driver_version,
        });
    }

    gpus
}
