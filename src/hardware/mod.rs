pub mod battery;
pub mod components;
pub mod cpu;
pub mod disks;
pub mod display_monitor;
pub mod gpu;
pub mod memory;
pub mod motherboard;
pub mod network;

use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
pub struct HardwareInfo {
    pub cpu: cpu::CpuInfo,
    pub memory: memory::MemoryInfo,
    pub disks: Vec<disks::DiskInfo>,
    pub networks: Vec<network::NetworkInterfaceInfo>,
    pub components: Vec<components::ComponentInfo>,
    pub batteries: Vec<battery::BatteryInfo>,
    pub motherboard: motherboard::MotherboardInfo,
    pub gpus: Vec<gpu::GpuInfo>,
    pub monitors: Vec<display_monitor::MonitorInfo>,
}

pub fn collect(sys: &System) -> HardwareInfo {
    HardwareInfo {
        cpu: cpu::collect(sys),
        memory: memory::collect(sys),
        disks: disks::collect(),
        networks: network::collect(),
        components: components::collect(),
        batteries: battery::collect(),
        motherboard: motherboard::collect(),
        gpus: gpu::collect(),
        monitors: display_monitor::collect(),
    }
}
