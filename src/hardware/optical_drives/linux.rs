use super::OpticalDriveInfo;
use std::fs;

/// Type de périphérique SCSI 5 = CD-ROM/DVD (voir include/scsi/scsi_proto.h).
const SCSI_TYPE_ROM: &str = "5";

pub fn collect() -> Vec<OpticalDriveInfo> {
    let mut drives = Vec::new();
    let Ok(entries) = fs::read_dir("/sys/block") else {
        return drives;
    };

    for entry in entries.filter_map(|e| e.ok()) {
        let name = entry.file_name().to_string_lossy().to_string();

        if name.starts_with("fd") {
            drives.push(OpticalDriveInfo {
                name: name.clone(),
                vendor: None,
                kind: "Disquette".to_string(),
            });
            continue;
        }

        let device_dir = entry.path().join("device");
        let Ok(scsi_type) = fs::read_to_string(device_dir.join("type")) else {
            continue;
        };
        if scsi_type.trim() != SCSI_TYPE_ROM {
            continue;
        }

        let vendor = fs::read_to_string(device_dir.join("vendor"))
            .ok()
            .map(|s| s.trim().to_string());
        let model = fs::read_to_string(device_dir.join("model"))
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| name.clone());

        drives.push(OpticalDriveInfo {
            name: model,
            vendor,
            kind: "CD/DVD".to_string(),
        });
    }

    drives
}
