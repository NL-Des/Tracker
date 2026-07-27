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

/// Extrait la vitesse de liaison en Mbps depuis une ligne `media:` de
/// `ifconfig` (ex. `"autoselect (1000baseT <full-duplex>)"` ou
/// `"1000baseT <full-duplex>"`). `None` si le format ne contient pas de
/// jeton `<N>base...` reconnu (ex. `"autoselect"` seul, interface inactive).
fn parse_media_speed(media_value: &str) -> Option<u64> {
    let inner = media_value
        .find('(')
        .and_then(|start| {
            let rest = &media_value[start + 1..];
            rest.find(')').map(|end| &rest[..end])
        })
        .unwrap_or(media_value);
    let token = inner.split_whitespace().next()?;
    let idx = token.find("base")?;
    token[..idx].parse::<u64>().ok()
}

/// Associe chaque interface à son type de port matériel via
/// `networksetup -listallhardwareports` (lecture libre), format par blocs :
/// "Hardware Port: Wi-Fi" / "Device: en0" / "Ethernet Address: ...".
fn hardware_port_types() -> HashMap<String, String> {
    let mut types = HashMap::new();
    let Some(text) = crate::command::run("networksetup", &["-listallhardwareports"]) else {
        return types;
    };

    let mut current_port: Option<String> = None;
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(port) = trimmed.strip_prefix("Hardware Port: ") {
            current_port = Some(port.to_string());
        } else if let Some(device) = trimmed.strip_prefix("Device: ") {
            if let Some(port) = &current_port {
                let connection_type = if port.contains("Wi-Fi") || port.contains("AirPort") {
                    "wifi"
                } else {
                    "wired"
                };
                types.insert(device.to_string(), connection_type.to_string());
            }
        }
    }

    types
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
        } else if let Some(rest) = trimmed.strip_prefix("media: ") {
            entry.link_speed_mbps = parse_media_speed(rest);
        }
    }

    let mut port_types = hardware_port_types();
    for (name, entry) in details.iter_mut() {
        if name == "lo0" {
            entry.connection_type = Some("loopback".to_string());
        } else if let Some(connection_type) = port_types.remove(name) {
            entry.connection_type = Some(connection_type);
        }
    }

    details
}

#[cfg(test)]
mod tests {
    use super::parse_media_speed;

    #[test]
    fn parses_gigabit_speed_with_duplex_suffix() {
        assert_eq!(parse_media_speed("autoselect (1000baseT <full-duplex>)"), Some(1000));
    }

    #[test]
    fn parses_speed_without_autoselect_wrapper() {
        assert_eq!(parse_media_speed("100baseTX <full-duplex>"), Some(100));
    }

    #[test]
    fn returns_none_for_autoselect_alone() {
        assert_eq!(parse_media_speed("autoselect"), None);
    }
}
