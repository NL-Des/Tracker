use super::ServiceInfo;

/// `launchctl list` en tant qu'utilisateur courant, lecture seule.
/// Format : "PID Status Label".
pub fn collect() -> Vec<ServiceInfo> {
    let Some(text) = crate::command::run("launchctl", &["list"]) else {
        return Vec::new();
    };

    text.lines()
        .skip(1)
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            let pid = fields.next()?;
            let status = fields.next()?;
            let name = fields.next()?.to_string();
            Some(ServiceInfo {
                name,
                status: format!("pid={pid} status={status}"),
            })
        })
        .collect()
}
