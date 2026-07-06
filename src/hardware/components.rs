use serde::Serialize;
use sysinfo::Components;

#[derive(Serialize)]
pub struct ComponentInfo {
    pub label: String,
    pub temperature_celsius: Option<f32>,
    pub max_temperature_celsius: Option<f32>,
    pub critical_temperature_celsius: Option<f32>,
}

pub fn collect() -> Vec<ComponentInfo> {
    let components = Components::new_with_refreshed_list();
    components
        .iter()
        .map(|component| ComponentInfo {
            label: component.label().to_string(),
            temperature_celsius: component.temperature(),
            max_temperature_celsius: component.max(),
            critical_temperature_celsius: component.critical(),
        })
        .collect()
}
