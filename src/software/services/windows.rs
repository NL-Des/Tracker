use super::ServiceInfo;
use serde::Deserialize;
use wmi::WMIConnection;

/// `Win32_Service` via WMI est en lecture seule, aucune élévation requise.
#[derive(Deserialize)]
#[serde(rename = "Win32_Service")]
struct Service {
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "State")]
    state: Option<String>,
}

pub fn collect() -> Vec<ServiceInfo> {
    let Ok(con) = WMIConnection::new() else {
        return Vec::new();
    };
    let Ok(services) = con.query::<Service>() else {
        return Vec::new();
    };

    services
        .into_iter()
        .filter_map(|s| {
            s.name.map(|name| ServiceInfo {
                name,
                status: s.state.unwrap_or_else(|| "?".to_string()),
            })
        })
        .collect()
}
