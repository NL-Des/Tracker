use serde::Serialize;
use sysinfo::Networks;

#[derive(Serialize)]
pub struct NetworkInterfaceInfo {
    pub interface_name: String,
    pub received_bytes: u64,
    pub transmitted_bytes: u64,
}

#[derive(Serialize)]
pub struct NetworkInfo {
    pub interfaces: Vec<NetworkInterfaceInfo>,
    /// Passerelle par défaut (`ip route`, lecture libre sur Linux).
    pub default_gateway: Option<String>,
    /// Serveurs DNS configurés (`/etc/resolv.conf`, lecture libre).
    pub dns_servers: Vec<String>,
}

fn collect_interfaces() -> Vec<NetworkInterfaceInfo> {
    let networks = Networks::new_with_refreshed_list();
    networks
        .iter()
        .map(|(interface_name, data)| NetworkInterfaceInfo {
            interface_name: interface_name.clone(),
            received_bytes: data.received(),
            transmitted_bytes: data.transmitted(),
        })
        .collect()
}

#[cfg(target_os = "linux")]
fn read_default_gateway_linux() -> Option<String> {
    let text = crate::command::run("ip", &["route", "show", "default"])?;
    // Format : "default via 192.168.1.1 dev eth0 ..."
    text.lines().find_map(|line| {
        let mut words = line.split_whitespace();
        while let Some(word) = words.next() {
            if word == "via" {
                return words.next().map(|s| s.to_string());
            }
        }
        None
    })
}

fn read_default_gateway() -> Option<String> {
    crate::os_dispatch::dispatch_os!(read_default_gateway_linux(), None, None, None)
}

#[cfg(target_os = "linux")]
fn read_dns_servers_linux() -> Vec<String> {
    let Ok(contents) = std::fs::read_to_string("/etc/resolv.conf") else {
        return Vec::new();
    };
    contents
        .lines()
        .filter_map(|line| line.trim().strip_prefix("nameserver"))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn read_dns_servers() -> Vec<String> {
    crate::os_dispatch::dispatch_os!(read_dns_servers_linux(), Vec::new(), Vec::new(), Vec::new())
}

pub fn collect() -> NetworkInfo {
    NetworkInfo {
        interfaces: collect_interfaces(),
        default_gateway: read_default_gateway(),
        dns_servers: read_dns_servers(),
    }
}
