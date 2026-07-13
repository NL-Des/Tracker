use super::ScheduledTaskInfo;

/// `crontab -l` sur son propre crontab ne nécessite pas de droits root.
/// Absence de crontab (code de sortie non nul) traitée comme une liste vide.
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
