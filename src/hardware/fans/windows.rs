use super::FanInfo;
use serde::Deserialize;
use wmi::WMIConnection;

/// `Win32_Fan` est souvent peu ou pas peuplée selon le support ACPI du
/// matériel : best effort, comme les autres heuristiques WMI de ce projet.
#[derive(Deserialize)]
#[serde(rename = "Win32_Fan")]
struct Fan {
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "DesiredSpeed")]
    desired_speed: Option<u32>,
}

pub fn collect() -> Vec<FanInfo> {
    let Ok(con) = WMIConnection::new() else {
        return Vec::new();
    };
    let Ok(fans) = con.query::<Fan>() else {
        return Vec::new();
    };

    fans.into_iter()
        .filter_map(|f| {
            f.name.map(|name| FanInfo {
                name,
                speed_rpm: f.desired_speed,
            })
        })
        .collect()
}
