use super::NetworkConnectionInfo;

/// `netstat -p tcp -p udp` sans sudo liste déjà les connexions accessibles
/// à l'utilisateur courant.
pub fn collect() -> Vec<NetworkConnectionInfo> {
    let Some(text) = crate::command::run("netstat", &["-an"]) else {
        return Vec::new();
    };

    text.lines()
        .filter(|line| line.starts_with("tcp") || line.starts_with("udp"))
        .filter_map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            let protocol = (*fields.first()?).to_string();
            let local_address = (*fields.get(3)?).to_string();
            let state = fields.get(5).map(|s| s.to_string()).unwrap_or_default();
            Some(NetworkConnectionInfo {
                protocol,
                local_address,
                state,
            })
        })
        .collect()
}
