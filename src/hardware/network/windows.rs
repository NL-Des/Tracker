use super::InterfaceDetails;
use serde::Deserialize;
use std::collections::HashMap;
use wmi::WMIConnection;

/// Comportement inchangé : pas de source lecture-libre équivalente identifiée
/// pour ce projet côté Windows.
pub fn default_gateway() -> Option<String> {
    None
}

pub fn dns_servers() -> Vec<String> {
    Vec::new()
}

#[derive(Deserialize)]
#[serde(rename = "Win32_NetworkAdapterConfiguration")]
struct AdapterConfig {
    #[serde(rename = "Description")]
    description: Option<String>,
    #[serde(rename = "MACAddress")]
    mac_address: Option<String>,
    #[serde(rename = "IPAddress")]
    ip_address: Option<Vec<String>>,
}

#[derive(Deserialize)]
#[serde(rename = "Win32_NetworkAdapter")]
struct Adapter {
    #[serde(rename = "NetConnectionID")]
    net_connection_id: Option<String>,
    #[serde(rename = "Description")]
    description: Option<String>,
    #[serde(rename = "Speed")]
    speed: Option<u64>,
    #[serde(rename = "AdapterType")]
    adapter_type: Option<String>,
}

fn empty_details() -> InterfaceDetails {
    InterfaceDetails {
        mac_address: None,
        ipv4_addresses: Vec::new(),
        ipv6_addresses: Vec::new(),
        link_speed_mbps: None,
        connection_type: None,
    }
}

/// `Win32_NetworkAdapterConfiguration` (MAC/IP) est jointe à
/// `Win32_NetworkAdapter` (vitesse/type) via la description commune, puis
/// republiée sous `NetConnectionID` (le nom d'interface visible côté
/// utilisateur, ex. "Ethernet", "Wi-Fi") pour correspondre aux noms renvoyés
/// par `sysinfo`. Lecture seule via WMI, aucune élévation requise.
pub fn all_details() -> HashMap<String, InterfaceDetails> {
    let mut details = HashMap::new();
    let Ok(con) = WMIConnection::new() else {
        return details;
    };

    let mut by_description: HashMap<String, InterfaceDetails> = HashMap::new();
    if let Ok(configs) = con.query::<AdapterConfig>() {
        for c in configs {
            let Some(desc) = c.description else { continue };
            let mut ipv4 = Vec::new();
            let mut ipv6 = Vec::new();
            for addr in c.ip_address.unwrap_or_default() {
                if addr.contains(':') {
                    ipv6.push(addr);
                } else {
                    ipv4.push(addr);
                }
            }
            by_description.insert(
                desc,
                InterfaceDetails {
                    mac_address: c.mac_address,
                    ipv4_addresses: ipv4,
                    ipv6_addresses: ipv6,
                    link_speed_mbps: None,
                    connection_type: None,
                },
            );
        }
    }

    if let Ok(adapters) = con.query::<Adapter>() {
        for a in adapters {
            let Some(name) = a.net_connection_id.clone() else { continue };
            let mut entry = a
                .description
                .as_ref()
                .and_then(|d| by_description.remove(d))
                .unwrap_or_else(empty_details);
            entry.link_speed_mbps = a.speed.map(|s| s / 1_000_000).filter(|v| *v > 0);
            entry.connection_type = a.adapter_type.map(|t| {
                if t.to_lowercase().contains("wireless") {
                    "wifi".to_string()
                } else {
                    "wired".to_string()
                }
            });
            details.insert(name, entry);
        }
    }

    details
}
