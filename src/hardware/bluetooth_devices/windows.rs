use super::BluetoothDeviceInfo;
use serde::Deserialize;
use wmi::WMIConnection;

#[derive(Deserialize)]
#[serde(rename = "Win32_PnPEntity")]
struct PnpEntity {
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "DeviceID")]
    device_id: Option<String>,
}

pub fn collect() -> Vec<BluetoothDeviceInfo> {
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
                .is_some_and(|id| id.starts_with("BTHENUM"))
        })
        .filter_map(|e| e.name)
        .map(|name| BluetoothDeviceInfo { name })
        .collect()
}
