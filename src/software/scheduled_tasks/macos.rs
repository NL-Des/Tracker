use super::ScheduledTaskInfo;
use std::process::Command;

/// Sur macOS, `cron`/`crontab -l` reste disponible pour l'utilisateur
/// courant (même mécanisme que Linux), en plus des `launchd` agents
/// utilisateur déjà couverts par `services.rs`.
pub fn collect() -> Vec<ScheduledTaskInfo> {
    let Ok(output) = Command::new("crontab").arg("-l").output() else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(&output.stdout);

    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            let schedule: Vec<&str> = (&mut fields).take(5).collect();
            if schedule.len() != 5 {
                return None;
            }
            let command: String = fields.collect::<Vec<_>>().join(" ");
            Some(ScheduledTaskInfo {
                name: command,
                schedule: schedule.join(" "),
            })
        })
        .collect()
}
