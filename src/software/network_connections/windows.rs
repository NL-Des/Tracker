use super::NetworkConnectionInfo;

/// `netstat -an` ne nécessite pas de droits admin pour les connexions de
/// l'utilisateur courant.
pub fn collect() -> Vec<NetworkConnectionInfo> {
    let Some(text) = crate::command::run("netstat", &["-an"]) else {
        return Vec::new();
    };

    text.lines()
        .filter_map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            let protocol = fields.first()?;
            if !matches!(*protocol, "TCP" | "UDP") {
                return None;
            }
            let local_address = (*fields.get(1)?).to_string();
            let state = fields.get(3).map(|s| s.to_string()).unwrap_or_default();
            Some(NetworkConnectionInfo {
                protocol: protocol.to_string(),
                local_address,
                state,
            })
        })
        .collect()
}
