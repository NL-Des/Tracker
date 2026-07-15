use super::InterfaceDetails;
use std::collections::HashMap;

/// Comportement inchangé : macOS n'a pas de source lecture-libre équivalente
/// à `ip route`/`resolv.conf` déjà identifiée pour ce projet.
pub fn default_gateway() -> Option<String> {
    None
}

pub fn dns_servers() -> Vec<String> {
    Vec::new()
}

/// `ifconfig` (sans argument) liste toutes les interfaces en un seul appel,
/// lecture libre. Parsing par bloc : une ligne non-indentée démarre une
/// nouvelle interface, les lignes indentées suivantes portent ses détails.
pub fn all_details() -> HashMap<String, InterfaceDetails> {
    let mut details: HashMap<String, InterfaceDetails> = HashMap::new();
    let Some(text) = crate::command::run("ifconfig", &[]) else {
        return details;
    };

    let mut current: Option<String> = None;
    for line in text.lines() {
        if !line.starts_with(char::is_whitespace) {
            let Some((name, _)) = line.split_once(':') else {
                current = None;
                continue;
            };
            let name = name.trim().to_string();
            details.entry(name.clone()).or_insert_with(|| InterfaceDetails {
                mac_address: None,
                ipv4_addresses: Vec::new(),
                ipv6_addresses: Vec::new(),
                link_speed_mbps: None,
                connection_type: None,
            });
            current = Some(name);
            continue;
        }

        let Some(name) = current.clone() else { continue };
        let Some(entry) = details.get_mut(&name) else { continue };
        let trimmed = line.trim();

        if let Some(rest) = trimmed.strip_prefix("ether ") {
            entry.mac_address = rest.split_whitespace().next().map(|s| s.to_string());
        } else if let Some(rest) = trimmed.strip_prefix("inet6 ") {
            if let Some(addr) = rest.split_whitespace().next() {
                entry.ipv6_addresses.push(addr.to_string());
            }
        } else if let Some(rest) = trimmed.strip_prefix("inet ") {
            if let Some(addr) = rest.split_whitespace().next() {
                entry.ipv4_addresses.push(addr.to_string());
            }
        }
    }

    details
}
