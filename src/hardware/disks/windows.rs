use serde::Deserialize;
use wmi::WMIConnection;

#[derive(Deserialize)]
#[serde(rename = "Win32_DiskPartition")]
struct DiskPartition {
    #[serde(rename = "DeviceID")]
    device_id: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename = "Win32_DiskDrive")]
struct DiskDrive {
    #[serde(rename = "Model")]
    model: Option<String>,
    #[serde(rename = "SerialNumber")]
    serial_number: Option<String>,
}

/// Remonte de la lettre de lecteur (ex. `C:`) au disque physique via les
/// associations WMI standard `Win32_LogicalDiskToPartition` puis
/// `Win32_DiskDriveToDiskPartition`, pour obtenir `Win32_DiskDrive.{Model,SerialNumber}`.
/// Aucune élévation requise pour ces requêtes en lecture seule.
pub fn read(device_name: &str) -> (Option<String>, Option<String>) {
    let drive_letter = device_name.trim_end_matches(['\\', '/']).to_string();
    let Ok(con) = WMIConnection::new() else {
        return (None, None);
    };

    let partition_query = format!(
        "ASSOCIATORS OF {{Win32_LogicalDisk.DeviceID='{drive_letter}'}} \
         WHERE AssocClass = Win32_LogicalDiskToPartition ResultClass = Win32_DiskPartition"
    );
    let Ok(partitions) = con.raw_query::<DiskPartition>(&partition_query) else {
        return (None, None);
    };
    let Some(partition_id) = partitions.into_iter().find_map(|p| p.device_id) else {
        return (None, None);
    };
    let partition_id_escaped = partition_id.replace('\\', "\\\\");

    let drive_query = format!(
        "ASSOCIATORS OF {{Win32_DiskPartition.DeviceID='{partition_id_escaped}'}} \
         WHERE AssocClass = Win32_DiskDriveToDiskPartition ResultClass = Win32_DiskDrive"
    );
    let Ok(drives) = con.raw_query::<DiskDrive>(&drive_query) else {
        return (None, None);
    };
    match drives.into_iter().next() {
        Some(drive) => (drive.model, drive.serial_number),
        None => (None, None),
    }
}
