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

/// Heuristique légère basée sur les IDs vendeur PCI (pas de base pci.ids
/// complète) : suffisant pour identifier les fabricants GPU courants.
pub fn collect() -> Vec<GpuInfo> {
    let mut gpus = Vec::new();
    let Ok(entries) = fs::read_dir("/sys/class/drm") else {
        return gpus;
    };

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

        gpus.push(GpuInfo {
            name: display_name,
            vendor,
        });
    }

    gpus
}
