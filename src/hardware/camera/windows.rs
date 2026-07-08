use super::CameraInfo;
use serde::Deserialize;
use wmi::WMIConnection;

#[derive(Deserialize)]
#[serde(rename = "Win32_PnPEntity")]
struct PnpEntity {
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "PNPClass")]
    pnp_class: Option<String>,
}

pub fn collect() -> Vec<CameraInfo> {
    let Ok(con) = WMIConnection::new() else {
        return Vec::new();
    };
    let Ok(entities) = con.query::<PnpEntity>() else {
        return Vec::new();
    };

    entities
        .into_iter()
        .filter(|e| {
            matches!(
                e.pnp_class.as_deref(),
                Some("Camera") | Some("Image")
            )
        })
        .filter_map(|e| e.name)
        .map(|name| CameraInfo { name })
        .collect()
}
