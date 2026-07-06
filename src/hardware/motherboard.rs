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
}

pub fn collect() -> MotherboardInfo {
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
}
