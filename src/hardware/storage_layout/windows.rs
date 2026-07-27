use super::{PartitionInfo, RaidArrayInfo, StorageLayoutInfo};
use serde::Deserialize;
use wmi::WMIConnection;

#[derive(Deserialize)]
#[serde(rename = "Win32_LogicalDisk")]
struct LogicalDisk {
    #[serde(rename = "DeviceID")]
    device_id: Option<String>,
    #[serde(rename = "FileSystem")]
    file_system: Option<String>,
    #[serde(rename = "Size")]
    size: Option<u64>,
}

#[derive(Deserialize)]
#[serde(rename = "MSFT_VirtualDisk")]
struct VirtualDisk {
    #[serde(rename = "FriendlyName")]
    friendly_name: Option<String>,
    #[serde(rename = "ResiliencySettingName")]
    resiliency_setting_name: Option<String>,
    #[serde(rename = "HealthStatus")]
    health_status: Option<u16>,
}

/// Traduction des codes `HealthStatus` de `MSFT_VirtualDisk` (namespace
/// `ROOT\Microsoft\Windows\Storage`) en libellé lisible.
fn health_status_label(code: Option<u16>) -> String {
    match code {
        Some(0) => "Healthy".to_string(),
        Some(1) => "Warning".to_string(),
        Some(2) => "Unhealthy".to_string(),
        _ => "?".to_string(),
    }
}

/// Storage Spaces (RAID logiciel Windows) via le namespace WMI dédié
/// `ROOT\Microsoft\Windows\Storage`, classe `MSFT_VirtualDisk` — même
/// pattern que `read_tpm_version()` dans `hardware/motherboard.rs`. Lecture
/// seule, aucune élévation requise pour l'énumération ; absence de Storage
/// Spaces configuré (cas le plus fréquent) renvoie simplement un `Vec` vide.
fn collect_raid() -> Vec<RaidArrayInfo> {
    let Ok(com_con) = wmi::COMLibrary::new() else {
        return Vec::new();
    };
    let Ok(con) =
        WMIConnection::with_namespace_path("ROOT\\Microsoft\\Windows\\Storage", com_con)
    else {
        return Vec::new();
    };
    let Ok(disks) = con.query::<VirtualDisk>() else {
        return Vec::new();
    };

    disks
        .into_iter()
        .filter_map(|d| {
            let device = d.friendly_name?;
            Some(RaidArrayInfo {
                device,
                level: d.resiliency_setting_name.unwrap_or_else(|| "?".to_string()),
                state: health_status_label(d.health_status),
                devices: Vec::new(),
            })
        })
        .collect()
}

/// `Win32_LogicalDisk` via WMI est en lecture seule, aucune élévation
/// requise. Pas de LVM générique exposé simplement sans admin sur Windows.
pub fn collect() -> StorageLayoutInfo {
    let Ok(con) = WMIConnection::new() else {
        return StorageLayoutInfo {
            partitions: Vec::new(),
            lvm_volumes: Vec::new(),
            raid_arrays: Vec::new(),
        };
    };

    let partitions = con
        .query::<LogicalDisk>()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|d| {
            let device = d.device_id?;
            Some(PartitionInfo {
                device,
                fs_type: d.file_system.unwrap_or_else(|| "?".to_string()),
                size_gb: d.size.map(|b| b / 1_000_000_000).unwrap_or(0),
            })
        })
        .collect();

    StorageLayoutInfo {
        partitions,
        lvm_volumes: Vec::new(),
        raid_arrays: collect_raid(),
    }
}
