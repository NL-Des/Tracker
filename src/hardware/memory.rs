use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
pub struct MemoryModuleInfo {
    pub manufacturer: Option<String>,
    pub serial_number: Option<String>,
    pub capacity_mb: Option<u64>,
    pub speed_mhz: Option<u64>,
}

#[derive(Serialize)]
pub struct MemoryInfo {
    pub total_mb: u64,
    pub used_mb: u64,
    pub total_swap_mb: u64,
    pub used_swap_mb: u64,
    /// Détail par barrette (fabricant, série, capacité, fréquence). Windows
    /// uniquement via WMI ; pas d'équivalent sysfs non privilégié sur Linux
    /// (nécessite `dmidecode`, root requis), ni sur macOS.
    pub modules: Vec<MemoryModuleInfo>,
}

#[derive(serde::Deserialize)]
#[cfg(target_os = "windows")]
#[serde(rename = "Win32_PhysicalMemory")]
struct WmiPhysicalMemory {
    #[serde(rename = "Manufacturer")]
    manufacturer: Option<String>,
    #[serde(rename = "SerialNumber")]
    serial_number: Option<String>,
    #[serde(rename = "Capacity")]
    capacity: Option<u64>,
    #[serde(rename = "Speed")]
    speed: Option<u64>,
}

#[cfg(target_os = "windows")]
fn read_modules() -> Vec<MemoryModuleInfo> {
    let Ok(con) = wmi::WMIConnection::new() else {
        return Vec::new();
    };
    let Ok(sticks) = con.query::<WmiPhysicalMemory>() else {
        return Vec::new();
    };
    sticks
        .into_iter()
        .map(|s| MemoryModuleInfo {
            manufacturer: s.manufacturer,
            serial_number: s.serial_number,
            capacity_mb: s.capacity.map(|c| c / 1024 / 1024),
            speed_mhz: s.speed,
        })
        .collect()
}

#[cfg(not(target_os = "windows"))]
fn read_modules() -> Vec<MemoryModuleInfo> {
    Vec::new()
}

pub fn collect(sys: &System) -> MemoryInfo {
    MemoryInfo {
        total_mb: sys.total_memory() / 1024 / 1024,
        used_mb: sys.used_memory() / 1024 / 1024,
        total_swap_mb: sys.total_swap() / 1024 / 1024,
        used_swap_mb: sys.used_swap() / 1024 / 1024,
        modules: read_modules(),
    }
}
