use super::KernelModuleInfo;
use serde::Deserialize;
use wmi::WMIConnection;

/// `Win32_SystemDriver` liste les pilotes système, lecture seule via WMI,
/// sans droits admin requis.
#[derive(Deserialize)]
#[serde(rename = "Win32_SystemDriver")]
struct SystemDriver {
    #[serde(rename = "Name")]
    name: Option<String>,
}

pub fn collect() -> Vec<KernelModuleInfo> {
    let Ok(con) = WMIConnection::new() else {
        return Vec::new();
    };
    let Ok(drivers) = con.query::<SystemDriver>() else {
        return Vec::new();
    };

    drivers
        .into_iter()
        .filter_map(|d| {
            d.name.map(|name| KernelModuleInfo {
                name,
                size_bytes: 0,
            })
        })
        .collect()
}
