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
pub struct VulnerabilityInfo {
    pub name: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct CpuInfo {
    pub architecture: String,
    pub core_count: usize,
    pub global_usage_percent: f32,
    pub cores: Vec<CoreInfo>,
    pub vulnerabilities: Vec<VulnerabilityInfo>,
    pub scaling_governor: Option<String>,
    pub processor_id: Option<String>,
    /// Champ `microcode` de `/proc/cpuinfo`, lecture libre sur Linux ;
    /// absent sur les autres OS (pas de source non privilégiée identifiée).
    pub microcode_version: Option<String>,
}

#[derive(serde::Deserialize)]
#[cfg(target_os = "windows")]
#[serde(rename = "Win32_Processor")]
struct WmiProcessor {
    #[serde(rename = "ProcessorId")]
    processor_id: Option<String>,
}

/// `ProcessorId` CPU (numéro de série logique du CPU) via WMI. Namespace WMI
/// par défaut (`ROOT\CIMV2`), aucune élévation requise.
#[cfg(target_os = "windows")]
fn read_processor_id() -> Option<String> {
    let con = wmi::WMIConnection::new().ok()?;
    let mut processors: Vec<WmiProcessor> = con.query().ok()?;
    processors.pop()?.processor_id
}

#[cfg(not(target_os = "windows"))]
fn read_processor_id() -> Option<String> {
    None
}

/// Version du microcode CPU actuellement chargé, lecture libre sur Linux
/// (`/proc/cpuinfo`) ; pas de source non privilégiée simple identifiée sur
/// les autres OS.
#[cfg(target_os = "linux")]
fn read_microcode_version() -> Option<String> {
    let text = std::fs::read_to_string("/proc/cpuinfo").ok()?;
    text.lines().find_map(|line| {
        let (key, value) = line.split_once(':')?;
        if key.trim() == "microcode" {
            Some(value.trim().to_string())
        } else {
            None
        }
    })
}

#[cfg(not(target_os = "linux"))]
fn read_microcode_version() -> Option<String> {
    None
}

/// Statuts des mitigations Spectre/Meltdown/etc. Lecture libre sur Linux
/// (aucune élévation requise) ; absent sur les autres OS.
#[cfg(target_os = "linux")]
fn read_vulnerabilities_linux() -> Vec<VulnerabilityInfo> {
    let Ok(entries) = std::fs::read_dir("/sys/devices/system/cpu/vulnerabilities") else {
        return Vec::new();
    };
    let mut vulnerabilities: Vec<VulnerabilityInfo> = entries
        .filter_map(|e| e.ok())
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().to_string();
            let status = std::fs::read_to_string(entry.path()).ok()?.trim().to_string();
            Some(VulnerabilityInfo { name, status })
        })
        .collect();
    vulnerabilities.sort_by(|a, b| a.name.cmp(&b.name));
    vulnerabilities
}

fn read_vulnerabilities() -> Vec<VulnerabilityInfo> {
    crate::os_dispatch::dispatch_os!(read_vulnerabilities_linux(), Vec::new(), Vec::new(), Vec::new())
}

/// Gouverneur de fréquence du premier cœur (performance/powersave/...).
/// Lecture libre sur Linux ; absent sur les autres OS.
#[cfg(target_os = "linux")]
fn read_scaling_governor_linux() -> Option<String> {
    std::fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn read_scaling_governor() -> Option<String> {
    crate::os_dispatch::dispatch_os!(read_scaling_governor_linux(), None, None, None)
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
        vulnerabilities: read_vulnerabilities(),
        scaling_governor: read_scaling_governor(),
        processor_id: read_processor_id(),
        microcode_version: read_microcode_version(),
    }
}
