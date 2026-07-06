use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
pub struct CoreInfo {
    pub index: usize,
    pub usage_percent: f32,
    pub frequency_mhz: u64,
    pub brand: String,
}

#[derive(Serialize)]
pub struct CpuInfo {
    pub architecture: String,
    pub core_count: usize,
    pub global_usage_percent: f32,
    pub cores: Vec<CoreInfo>,
}

pub fn collect(sys: &System) -> CpuInfo {
    CpuInfo {
        architecture: System::cpu_arch(),
        core_count: sys.cpus().len(),
        global_usage_percent: sys.global_cpu_usage(),
        cores: sys
            .cpus()
            .iter()
            .enumerate()
            .map(|(index, cpu)| CoreInfo {
                index,
                usage_percent: cpu.cpu_usage(),
                frequency_mhz: cpu.frequency(),
                brand: cpu.brand().to_string(),
            })
            .collect(),
    }
}
