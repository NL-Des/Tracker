use super::NetworkConnectionInfo;
use std::process::Command;

/// `ss -tunp` en mode utilisateur liste déjà les connexions du propre
/// utilisateur sans élévation (les connexions d'autres utilisateurs sont
/// simplement omises par le noyau, pas une erreur).
pub fn collect() -> Vec<NetworkConnectionInfo> {
    let Ok(output) = Command::new("ss").args(["-tun"]).output() else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(&output.stdout);

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
