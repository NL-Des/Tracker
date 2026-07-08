use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage_percent: f32,
    pub memory_mb: u64,
}

#[derive(Serialize)]
pub struct ProcessSummary {
    pub total_count: usize,
    pub processes: Vec<ProcessInfo>,
}

pub fn collect(sys: &System) -> ProcessSummary {
    let mut processes: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        .map(|(pid, process)| ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string_lossy().to_string(),
            cpu_usage_percent: process.cpu_usage(),
            memory_mb: process.memory() / 1024 / 1024,
        })
        .collect();
    processes.sort_by(|a, b| b.cpu_usage_percent.total_cmp(&a.cpu_usage_percent));

    ProcessSummary {
        total_count: sys.processes().len(),
        processes,
    }
}
