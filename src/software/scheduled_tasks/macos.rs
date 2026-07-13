use super::ScheduledTaskInfo;

/// Sur macOS, `cron`/`crontab -l` reste disponible pour l'utilisateur
/// courant (même mécanisme que Linux), en plus des `launchd` agents
/// utilisateur déjà couverts par `services.rs`.
pub fn collect() -> Vec<ScheduledTaskInfo> {
    let Some(text) = crate::command::run("crontab", &["-l"]) else {
        return Vec::new();
    };

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
