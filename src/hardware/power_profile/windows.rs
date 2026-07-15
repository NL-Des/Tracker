use super::PowerProfileInfo;

/// `powercfg /getactivescheme` est en lecture seule ; seule la modification
/// d'un plan d'alimentation nécessiterait une élévation. Format :
/// "Power Scheme GUID: ... (Balanced)".
pub fn collect() -> PowerProfileInfo {
    let Some(text) = crate::command::run("powercfg", &["/getactivescheme"]) else {
        return PowerProfileInfo { profile: None, sleep_mode: None };
    };

    let profile = text
        .find('(')
        .and_then(|start| text.find(')').map(|end| (start, end)))
        .filter(|(start, end)| end > start)
        .map(|(start, end)| text[start + 1..end].to_string());

    PowerProfileInfo { profile, sleep_mode: None }
}
