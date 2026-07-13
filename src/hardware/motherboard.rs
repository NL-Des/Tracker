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
    // Statut de sortie non vérifié ici (comportement existant conservé).
    let text = crate::command::run_lenient("mokutil", &["--sb-state"])?;
    text.lines().next().map(|line| line.trim().to_string())
}

#[cfg(not(target_os = "linux"))]
fn read_secure_boot_state() -> Option<String> {
    None
}

pub fn collect() -> MotherboardInfo {
    let mut info = crate::os_dispatch::dispatch_os!(
        linux::collect(),
        macos::collect(),
        windows::collect(),
        MotherboardInfo::default()
    );
    info.secure_boot = read_secure_boot_state();
    info
}
