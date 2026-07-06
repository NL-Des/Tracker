use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
pub struct MemoryInfo {
    pub total_mb: u64,
    pub used_mb: u64,
    pub total_swap_mb: u64,
    pub used_swap_mb: u64,
}

pub fn collect(sys: &System) -> MemoryInfo {
    MemoryInfo {
        total_mb: sys.total_memory() / 1024 / 1024,
        used_mb: sys.used_memory() / 1024 / 1024,
        total_swap_mb: sys.total_swap() / 1024 / 1024,
        used_swap_mb: sys.used_swap() / 1024 / 1024,
    }
}
