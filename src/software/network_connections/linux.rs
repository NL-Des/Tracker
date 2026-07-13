use super::NetworkConnectionInfo;

/// `ss -tunp` en mode utilisateur liste déjà les connexions du propre
/// utilisateur sans élévation (les connexions d'autres utilisateurs sont
/// simplement omises par le noyau, pas une erreur).
pub fn collect() -> Vec<NetworkConnectionInfo> {
    let Some(text) = crate::command::run("ss", &["-tun"]) else {
        return Vec::new();
    };

    text.lines()
        .skip(1)
        .filter_map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            let protocol = (*fields.first()?).to_string();
            let state = (*fields.get(1)?).to_string();
            let local_address = (*fields.get(4)?).to_string();
            Some(NetworkConnectionInfo {
                protocol,
                local_address,
                state,
            })
        })
        .collect()
}
