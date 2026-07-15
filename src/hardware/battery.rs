use serde::Serialize;
use starship_battery::units::ratio::percent;
use starship_battery::units::thermodynamic_temperature::degree_celsius;
use starship_battery::units::time::minute;

#[derive(Serialize)]
pub struct BatteryInfo {
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub state: String,
    pub technology: String,
    pub state_of_charge_percent: f32,
    pub state_of_health_percent: f32,
    pub serial_number: Option<String>,
    pub temperature_celsius: Option<f32>,
    pub cycle_count: Option<u32>,
    pub time_to_full_minutes: Option<f32>,
    pub time_to_empty_minutes: Option<f32>,
}

/// Infaillible par design : aucune batterie détectée (desktop) ou erreur
/// d'accès matériel renvoient simplement un `Vec` vide.
pub fn collect() -> Vec<BatteryInfo> {
    let manager = match starship_battery::Manager::new() {
        Ok(m) => m,
        Err(_) => return Vec::new(),
    };
    let batteries = match manager.batteries() {
        Ok(b) => b,
        Err(_) => return Vec::new(),
    };

    batteries
        .filter_map(|b| b.ok())
        .map(|battery| BatteryInfo {
            vendor: battery.vendor().map(|s| s.to_string()),
            model: battery.model().map(|s| s.to_string()),
            state: battery.state().to_string(),
            technology: battery.technology().to_string(),
            state_of_charge_percent: battery.state_of_charge().get::<percent>(),
            state_of_health_percent: battery.state_of_health().get::<percent>(),
            serial_number: battery
                .serial_number()
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty()),
            temperature_celsius: battery.temperature().map(|t| t.get::<degree_celsius>()),
            cycle_count: battery.cycle_count(),
            time_to_full_minutes: battery.time_to_full().map(|t| t.get::<minute>()),
            time_to_empty_minutes: battery.time_to_empty().map(|t| t.get::<minute>()),
        })
        .collect()
}
