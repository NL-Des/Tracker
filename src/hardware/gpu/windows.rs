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
    #[serde(rename = "AdapterRAM")]
    adapter_ram: Option<u64>,
    #[serde(rename = "DriverVersion")]
    driver_version: Option<String>,
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
                // `AdapterRAM` est un compteur 32 bits côté WMI : peu fiable
                // au-delà de 4 Go (limitation connue, cf. bilan.md).
                vram_mb: c.adapter_ram.map(|bytes| bytes / 1_000_000),
                driver_version: c.driver_version,
            })
        })
        .collect()
}
