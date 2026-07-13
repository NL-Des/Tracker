use super::ServiceInfo;

/// `systemctl list-units` en mode utilisateur est en lecture seule.
pub fn collect() -> Vec<ServiceInfo> {
    let Some(text) = crate::command::run(
        "systemctl",
        &[
            "list-units",
            "--type=service",
            "--all",
            "--no-legend",
            "--no-pager",
            "--plain",
        ],
    ) else {
        return Vec::new();
    };

    text.lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            let name = fields.next()?.to_string();
            // colonnes : LOAD ACTIVE SUB DESCRIPTION...
            let _load = fields.next()?;
            let active = fields.next()?;
            let sub = fields.next().unwrap_or("");
            Some(ServiceInfo {
                name,
                status: format!("{active}/{sub}"),
            })
        })
        .collect()
}
