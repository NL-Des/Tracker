#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize, Default)]
pub struct MotherboardInfo {
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub version: Option<String>,
    pub bios_vendor: Option<String>,
    pub bios_version: Option<String>,
    pub bios_date: Option<String>,
    pub machine_uuid: Option<String>,
    /// État Secure Boot ("enabled"/"disabled"), via `mokutil --sb-state`.
    /// Lecture libre sur la plupart des distributions Linux, aucune
    /// élévation requise.
    pub secure_boot: Option<String>,
}

#[cfg(target_os = "linux")]
fn read_secure_boot_state() -> Option<String> {
    let output = std::process::Command::new("mokutil")
        .arg("--sb-state")
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&output.stdout);
    text.lines().next().map(|line| line.trim().to_string())
}

#[cfg(not(target_os = "linux"))]
fn read_secure_boot_state() -> Option<String> {
    None
}

pub fn collect() -> MotherboardInfo {
    let mut info = {
        #[cfg(target_os = "linux")]
        {
            linux::collect()
        }
        #[cfg(target_os = "windows")]
        {
            windows::collect()
        }
        #[cfg(target_os = "macos")]
        {
            macos::collect()
        }
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            MotherboardInfo::default()
        }
    };
    info.secure_boot = read_secure_boot_state();
    info
}
