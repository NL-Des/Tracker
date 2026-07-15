use super::{PartitionInfo, StorageLayoutInfo};
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

/// `Win32_LogicalDisk` via WMI est en lecture seule, aucune élévation
/// requise. Pas de LVM ni de RAID logiciel générique exposé simplement sans
/// admin (Storage Spaces nécessiterait des classes WMI plus complexes, hors
/// périmètre pour l'instant).
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
        raid_arrays: Vec::new(),
    }
}
