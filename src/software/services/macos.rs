use super::ServiceInfo;
use std::process::Command;

/// `launchctl list` en tant qu'utilisateur courant, lecture seule.
/// Format : "PID Status Label".
pub fn collect() -> Vec<ServiceInfo> {
    let Ok(output) = Command::new("launchctl").arg("list").output() else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(&output.stdout);

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
