use super::GpuInfo;
use serde::Deserialize;
use wmi::WMIConnection;

#[derive(Deserialize)]
#[serde(rename = "Win32_VideoController")]
struct VideoController {
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "AdapterCompatibility")]
    adapter_compatibility: Option<String>,
}

pub fn collect() -> Vec<GpuInfo> {
    let Ok(con) = WMIConnection::new() else {
        return Vec::new();
    };
    let Ok(controllers) = con.query::<VideoController>() else {
        return Vec::new();
    };

    controllers
        .into_iter()
        .filter_map(|c| {
            c.name.map(|name| GpuInfo {
                name,
                vendor: c.adapter_compatibility,
            })
        })
        .collect()
}
