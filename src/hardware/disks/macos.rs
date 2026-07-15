fn parse_field(output: &str, key: &str) -> Option<String> {
    output.lines().find_map(|line| {
        let line = line.trim();
        line.strip_prefix(key)?
            .strip_prefix(':')
            .map(|v| v.trim().to_string())
            .filter(|s| !s.is_empty())
    })
}

/// `diskutil info` est non privilégié. `device_name` est le nom sysinfo du
/// disque (ex. `/dev/disk0s1`) ; `diskutil` accepte aussi bien le device de
/// partition que le disque entier. Note : `diskutil info` n'expose pas
/// toujours un vrai numéro de série physique pour les partitions/volumes
/// (dépend du contrôleur) — `serial` reste `None` dans ce cas plutôt que
/// d'inventer une valeur.
pub fn read(device_name: &str) -> (Option<String>, Option<String>) {
    let base_name = device_name.rsplit('/').next().unwrap_or(device_name);
    let Some(text) = crate::command::run("diskutil", &["info", base_name]) else {
        return (None, None);
    };

    let model = parse_field(&text, "Device / Media Name");
    let serial = parse_field(&text, "Disk / Partition UUID")
        .or_else(|| parse_field(&text, "Volume UUID"));

    (model, serial)
}
