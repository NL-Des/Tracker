#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

mod edid;

use display_info::DisplayInfo;
use serde::Serialize;

#[derive(Serialize)]
pub struct MonitorInfo {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub scale_factor: f32,
    pub frequency_hz: f32,
    pub is_primary: bool,
    pub is_builtin: bool,
    /// Identifiants EDID (fabricant PNP/modèle/série), best-effort : lecture
    /// libre sur Linux (`/sys/class/drm/*/edid`) et Windows (WMI
    /// `WmiMonitorID`) ; pas de source non privilégiée identifiée sur macOS.
    /// L'association à l'écran correspondant se fait par index (pas
    /// d'identifiant commun garanti entre `display-info` et les sources EDID).
    pub edid_vendor: Option<String>,
    pub edid_model: Option<String>,
    pub edid_serial_number: Option<String>,
}

/// Infaillible par design : absence de source EDID accessible (macOS, ou
/// environnement sans accès sysfs/WMI) renvoie simplement un `Vec` vide.
fn read_edid_ids() -> Vec<edid::EdidIds> {
    crate::os_dispatch::dispatch_os!(linux::read_all(), Vec::new(), windows::read_all(), Vec::new())
}

/// Infaillible par design : environnement headless/SSH ou Wayland non
/// supporté renvoient simplement un `Vec` vide.
pub fn collect() -> Vec<MonitorInfo> {
    let mut edid_ids = read_edid_ids().into_iter();
    DisplayInfo::all()
        .unwrap_or_default()
        .into_iter()
        .map(|display| {
            let ids = edid_ids.next();
            MonitorInfo {
                name: display.name,
                width: display.width,
                height: display.height,
                x: display.x,
                y: display.y,
                scale_factor: display.scale_factor,
                frequency_hz: display.frequency,
                is_primary: display.is_primary,
                is_builtin: display.is_builtin,
                edid_vendor: ids.as_ref().and_then(|i| i.vendor.clone()),
                edid_model: ids.as_ref().and_then(|i| i.model.clone()),
                edid_serial_number: ids.and_then(|i| i.serial_number),
            }
        })
        .collect()
}
