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
    crate::os_dispatch::dispatch_os!(
        linux::collect(),
        macos::collect(),
        windows::collect(),
        InputDevices {
            mice: Vec::new(),
            gamepads: Vec::new(),
            touchpads: Vec::new(),
        }
    )
}
