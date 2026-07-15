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
    /// Débit instantané mesuré par double échantillonnage des compteurs
    /// cumulés (`sysinfo`) espacé d'un court intervalle, plutôt qu'un débit
    /// moyen depuis le démarrage. Aucune commande externe, cross-plateforme.
    pub throughput_rx_mbps: Option<f64>,
    pub throughput_tx_mbps: Option<f64>,
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

/// Intervalle d'échantillonnage pour le calcul du débit instantané. Assez
/// court pour ne pas ralentir sensiblement la collecte, assez long pour
/// lisser le bruit d'un échantillon trop bref.
const THROUGHPUT_SAMPLE_INTERVAL: std::time::Duration = std::time::Duration::from_millis(300);

fn collect_interfaces() -> Vec<NetworkInterfaceInfo> {
    let mut networks = Networks::new_with_refreshed_list();
    let before: HashMap<String, (u64, u64)> = networks
        .iter()
        .map(|(name, data)| (name.clone(), (data.received(), data.transmitted())))
        .collect();

    std::thread::sleep(THROUGHPUT_SAMPLE_INTERVAL);
    networks.refresh(true);

    let mut details: HashMap<String, InterfaceDetails> = crate::os_dispatch::dispatch_os!(
        linux::all_details(),
        macos::all_details(),
        windows::all_details(),
        HashMap::new()
    );

    let interval_secs = THROUGHPUT_SAMPLE_INTERVAL.as_secs_f64();

    networks
        .iter()
        .map(|(interface_name, data)| {
            let d = details.remove(interface_name).unwrap_or_else(empty_details);
            let (rx_mbps, tx_mbps) = match before.get(interface_name) {
                Some(&(rx_before, tx_before)) => (
                    mbps_delta(rx_before, data.received(), interval_secs),
                    mbps_delta(tx_before, data.transmitted(), interval_secs),
                ),
                None => (None, None),
            };
            NetworkInterfaceInfo {
                interface_name: interface_name.clone(),
                received_bytes: data.received(),
                transmitted_bytes: data.transmitted(),
                mac_address: d.mac_address,
                ipv4_addresses: d.ipv4_addresses,
                ipv6_addresses: d.ipv6_addresses,
                link_speed_mbps: d.link_speed_mbps,
                connection_type: d.connection_type,
                throughput_rx_mbps: rx_mbps,
                throughput_tx_mbps: tx_mbps,
            }
        })
        .collect()
}

/// Convertit un delta d'octets cumulés sur `interval_secs` en Mbps.
/// `None` en cas de compteur qui repart à zéro (interface réinitialisée
/// entre les deux échantillons) plutôt qu'une valeur négative absurde.
fn mbps_delta(before: u64, after: u64, interval_secs: f64) -> Option<f64> {
    let delta_bytes = after.checked_sub(before)?;
    Some((delta_bytes as f64 * 8.0) / interval_secs / 1_000_000.0)
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
