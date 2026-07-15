use super::InterfaceDetails;
use std::collections::HashMap;
use std::fs;

/// Format : "default via 192.168.1.1 dev eth0 ...".
pub fn default_gateway() -> Option<String> {
    let text = crate::command::run("ip", &["route", "show", "default"])?;
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

pub fn dns_servers() -> Vec<String> {
    let Ok(contents) = fs::read_to_string("/etc/resolv.conf") else {
        return Vec::new();
    };
    contents
        .lines()
        .filter_map(|line| line.trim().strip_prefix("nameserver"))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Adresse MAC, vitesse de liaison et type de connexion via `/sys/class/net`
/// (lecture libre) ; adresses IPv4/IPv6 via `ip -o addr show` (lecture libre,
/// aucun droit particulier requis).
pub fn all_details() -> HashMap<String, InterfaceDetails> {
    let mut details: HashMap<String, InterfaceDetails> = HashMap::new();

    if let Ok(entries) = fs::read_dir("/sys/class/net") {
        for entry in entries.filter_map(|e| e.ok()) {
            let name = entry.file_name().to_string_lossy().to_string();
            let base = entry.path();

            let mac_address = fs::read_to_string(base.join("address"))
                .ok()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty() && s != "00:00:00:00:00:00");

            let link_speed_mbps = fs::read_to_string(base.join("speed"))
                .ok()
                .and_then(|s| s.trim().parse::<i64>().ok())
                .filter(|v| *v > 0)
                .map(|v| v as u64);

            let connection_type = if base.join("wireless").is_dir() {
                Some("wifi".to_string())
            } else if name == "lo" {
                Some("loopback".to_string())
            } else {
                Some("wired".to_string())
            };

            details.insert(
                name,
                InterfaceDetails {
                    mac_address,
                    ipv4_addresses: Vec::new(),
                    ipv6_addresses: Vec::new(),
                    link_speed_mbps,
                    connection_type,
                },
            );
        }
    }

    if let Some(text) = crate::command::run("ip", &["-o", "addr", "show"]) {
        // Format : "2: eth0    inet 192.168.1.10/24 brd ... scope global eth0..."
        for line in text.lines() {
            let mut fields = line.split_whitespace();
            let Some(_index) = fields.next() else { continue };
            let Some(name) = fields.next() else { continue };
            let name = name.trim_end_matches(':').to_string();
            let Some(family) = fields.next() else { continue };
            let Some(addr) = fields.next() else { continue };
            let addr_only = addr.split('/').next().unwrap_or(addr).to_string();

            let entry = details.entry(name).or_insert_with(|| InterfaceDetails {
                mac_address: None,
                ipv4_addresses: Vec::new(),
                ipv6_addresses: Vec::new(),
                link_speed_mbps: None,
                connection_type: None,
            });

            match family {
                "inet" => entry.ipv4_addresses.push(addr_only),
                "inet6" => entry.ipv6_addresses.push(addr_only),
                _ => {}
            }
        }
    }

    details
}
