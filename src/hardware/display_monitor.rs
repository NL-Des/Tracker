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
}

/// Infaillible par design : environnement headless/SSH ou Wayland non
/// supporté renvoient simplement un `Vec` vide.
pub fn collect() -> Vec<MonitorInfo> {
    DisplayInfo::all()
        .unwrap_or_default()
        .into_iter()
        .map(|display| MonitorInfo {
            name: display.name,
            width: display.width,
            height: display.height,
            x: display.x,
            y: display.y,
            scale_factor: display.scale_factor,
            frequency_hz: display.frequency,
            is_primary: display.is_primary,
        })
        .collect()
}
