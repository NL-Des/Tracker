use super::UsbDeviceInfo;
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
    #[serde(rename = "Manufacturer")]
    manufacturer: Option<String>,
}

pub fn collect() -> Vec<UsbDeviceInfo> {
    let Ok(con) = WMIConnection::new() else {
        return Vec::new();
    };
    let Ok(entities) = con.query::<PnpEntity>() else {
        return Vec::new();
    };

    entities
        .into_iter()
        .filter(|e| {
            e.device_id
                .as_deref()
                .is_some_and(|id| id.starts_with("USB\\"))
                && e.pnp_class.as_deref() != Some("USB")
        })
        .filter_map(|e| {
            e.name.map(|name| UsbDeviceInfo {
                name,
                vendor: e.manufacturer,
                device_class: e.pnp_class,
            })
        })
        .collect()
}
