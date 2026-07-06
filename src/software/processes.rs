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
    /// Processus utilisant plus de 5% de CPU au moment de la mesure.
    pub high_cpu_processes: Vec<ProcessInfo>,
}

pub fn collect(sys: &System) -> ProcessSummary {
    let high_cpu_processes = sys
        .processes()
        .iter()
        .filter(|(_, process)| process.cpu_usage() > 5.0)
        .map(|(pid, process)| ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string_lossy().to_string(),
            cpu_usage_percent: process.cpu_usage(),
            memory_mb: process.memory() / 1024 / 1024,
        })
        .collect();

    ProcessSummary {
        total_count: sys.processes().len(),
        high_cpu_processes,
    }
}
