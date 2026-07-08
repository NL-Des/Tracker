use super::PciDeviceInfo;
use serde::Deserialize;
use wmi::WMIConnection;

#[derive(Deserialize)]
#[serde(rename = "Win32_PnPEntity")]
struct PnpEntity {
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "DeviceID")]
    device_id: Option<String>,
    #[serde(rename = "PNPClass")]
    pnp_class: Option<String>,
}

pub fn collect() -> Vec<PciDeviceInfo> {
    let Ok(con) = WMIConnection::new() else {
        return Vec::new();
    };
    let Ok(entities) = con.query::<PnpEntity>() else {
        return Vec::new();
    };

    entities
        .into_iter()
        .filter(|e| e.device_id.as_deref().is_some_and(|id| id.starts_with("PCI\\")))
        .filter_map(|e| {
            e.name.map(|name| PciDeviceInfo {
                name,
                class: e.pnp_class.unwrap_or_else(|| "PCI".to_string()),
            })
        })
        .collect()
}
