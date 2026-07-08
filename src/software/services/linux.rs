use super::ServiceInfo;
use std::process::Command;

/// `systemctl list-units` en mode utilisateur est en lecture seule.
pub fn collect() -> Vec<ServiceInfo> {
    let Ok(output) = Command::new("systemctl")
        .args([
            "list-units",
            "--type=service",
            "--all",
            "--no-legend",
            "--no-pager",
            "--plain",
        ])
        .output()
    else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(&output.stdout);

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
