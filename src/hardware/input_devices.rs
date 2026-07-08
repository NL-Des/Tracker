#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct InputDeviceInfo {
    pub name: String,
}

pub struct InputDevices {
    pub mice: Vec<InputDeviceInfo>,
    pub gamepads: Vec<InputDeviceInfo>,
    pub touchpads: Vec<InputDeviceInfo>,
}

/// Infaillible par design : absence de périphérique ou erreur d'accès
/// matériel renvoient simplement des `Vec` vides.
pub fn collect() -> InputDevices {
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
        InputDevices {
            mice: Vec::new(),
            gamepads: Vec::new(),
            touchpads: Vec::new(),
        }
    }
}
