use super::ScheduledTaskInfo;
use serde::Deserialize;
use wmi::WMIConnection;

/// `MSFT_ScheduledTask` (namespace `root/Microsoft/Windows/TaskScheduler`)
/// est accessible en lecture sans droits admin pour lister les tâches.
#[derive(Deserialize)]
#[serde(rename = "MSFT_ScheduledTask")]
struct ScheduledTask {
    #[serde(rename = "TaskName")]
    task_name: Option<String>,
    #[serde(rename = "State")]
    state: Option<u8>,
}

fn connect() -> Option<WMIConnection> {
    let com_lib = wmi::COMLibrary::new().ok()?;
    WMIConnection::with_namespace_path("root\\Microsoft\\Windows\\TaskScheduler", com_lib).ok()
}

pub fn collect() -> Vec<ScheduledTaskInfo> {
    let Some(con) = connect() else {
        return Vec::new();
    };
    let Ok(tasks) = con.query::<ScheduledTask>() else {
        return Vec::new();
    };

    tasks
        .into_iter()
        .filter_map(|t| {
            t.task_name.map(|name| ScheduledTaskInfo {
                name,
                schedule: t
                    .state
                    .map(|s| format!("state={s}"))
                    .unwrap_or_else(|| "?".to_string()),
            })
        })
        .collect()
}
