use super::OpticalDriveInfo;
use serde::Deserialize;
use wmi::WMIConnection;

#[derive(Deserialize)]
#[serde(rename = "Win32_CDROMDrive")]
struct CdRomDrive {
    #[serde(rename = "Caption")]
    caption: Option<String>,
    #[serde(rename = "Manufacturer")]
    manufacturer: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename = "Win32_FloppyDrive")]
struct FloppyDrive {
    #[serde(rename = "Caption")]
    caption: Option<String>,
}

pub fn collect() -> Vec<OpticalDriveInfo> {
    let mut drives = Vec::new();

    let Ok(con) = WMIConnection::new() else {
        return drives;
    };

    if let Ok(cdroms) = con.query::<CdRomDrive>() {
        for cdrom in cdroms {
            drives.push(OpticalDriveInfo {
                name: cdrom.caption.unwrap_or_else(|| "Lecteur CD/DVD".to_string()),
                vendor: cdrom.manufacturer,
                kind: "CD/DVD".to_string(),
            });
        }
    }

    if let Ok(floppies) = con.query::<FloppyDrive>() {
        for floppy in floppies {
            drives.push(OpticalDriveInfo {
                name: floppy
                    .caption
                    .unwrap_or_else(|| "Lecteur de disquette".to_string()),
                vendor: None,
                kind: "Disquette".to_string(),
            });
        }
    }

    drives
}
