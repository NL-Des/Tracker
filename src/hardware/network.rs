#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;
use std::collections::HashMap;
use sysinfo::Networks;

#[derive(Serialize)]
pub struct NetworkInterfaceInfo {
    pub interface_name: String,
    pub received_bytes: u64,
    pub transmitted_bytes: u64,
    pub mac_address: Option<String>,
    pub ipv4_addresses: Vec<String>,
    pub ipv6_addresses: Vec<String>,
    pub link_speed_mbps: Option<u64>,
    pub connection_type: Option<String>,
}

#[derive(Serialize)]
pub struct NetworkInfo {
    pub interfaces: Vec<NetworkInterfaceInfo>,
    /// Passerelle par défaut (`ip route`, lecture libre sur Linux).
    pub default_gateway: Option<String>,
    /// Serveurs DNS configurés (`/etc/resolv.conf`, lecture libre).
    pub dns_servers: Vec<String>,
}

/// Détails complémentaires par interface (MAC, IP, vitesse de liaison, type
/// de connexion), obtenus séparément des compteurs `sysinfo` car les sources
/// diffèrent radicalement par OS (sysfs/`ip` sur Linux, `ifconfig` sur macOS,
/// WMI sur Windows).
struct InterfaceDetails {
    mac_address: Option<String>,
    ipv4_addresses: Vec<String>,
    ipv6_addresses: Vec<String>,
    link_speed_mbps: Option<u64>,
    connection_type: Option<String>,
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

fn collect_interfaces() -> Vec<NetworkInterfaceInfo> {
    let networks = Networks::new_with_refreshed_list();
    let mut details: HashMap<String, InterfaceDetails> = crate::os_dispatch::dispatch_os!(
        linux::all_details(),
        macos::all_details(),
        windows::all_details(),
        HashMap::new()
    );

    networks
        .iter()
        .map(|(interface_name, data)| {
            let d = details.remove(interface_name).unwrap_or_else(empty_details);
            NetworkInterfaceInfo {
                interface_name: interface_name.clone(),
                received_bytes: data.received(),
                transmitted_bytes: data.transmitted(),
                mac_address: d.mac_address,
                ipv4_addresses: d.ipv4_addresses,
                ipv6_addresses: d.ipv6_addresses,
                link_speed_mbps: d.link_speed_mbps,
                connection_type: d.connection_type,
            }
        })
        .collect()
}

fn read_default_gateway() -> Option<String> {
    crate::os_dispatch::dispatch_os!(
        linux::default_gateway(),
        macos::default_gateway(),
        windows::default_gateway(),
        None
    )
}

fn read_dns_servers() -> Vec<String> {
    crate::os_dispatch::dispatch_os!(
        linux::dns_servers(),
        macos::dns_servers(),
        windows::dns_servers(),
        Vec::new()
    )
}

pub fn collect() -> NetworkInfo {
    NetworkInfo {
        interfaces: collect_interfaces(),
        default_gateway: read_default_gateway(),
        dns_servers: read_dns_servers(),
    }
}
