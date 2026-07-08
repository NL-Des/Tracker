use super::{InputDeviceInfo, InputDevices};
use serde::Deserialize;
use wmi::WMIConnection;

/// `PointingType` distingue nativement souris (3) et touchpad (7) — pas
/// besoin d'heuristique sur le nom ici, contrairement à Linux/macOS.
const POINTING_TYPE_TOUCH_PAD: u32 = 7;

#[derive(Deserialize)]
#[serde(rename = "Win32_PointingDevice")]
struct PointingDevice {
    #[serde(rename = "Description")]
    description: Option<String>,
    #[serde(rename = "PointingType")]
    pointing_type: Option<u32>,
}

/// Pas de classe WMI dédiée fiable pour les manettes : on filtre les entrées
/// PnP dont le nom évoque une manette (best effort, comme les autres
/// heuristiques Windows de ce projet).
#[derive(Deserialize)]
#[serde(rename = "Win32_PnPEntity")]
struct PnpEntity {
    #[serde(rename = "Name")]
    name: Option<String>,
}

fn looks_like_gamepad(name: &str) -> bool {
    let lower = name.to_lowercase();
    ["controller", "gamepad", "joystick"]
        .iter()
        .any(|kw| lower.contains(kw))
}

pub fn collect() -> InputDevices {
    let empty = InputDevices {
        mice: Vec::new(),
        gamepads: Vec::new(),
        touchpads: Vec::new(),
    };
    let Ok(con) = WMIConnection::new() else {
        return empty;
    };

    let pointing_devices: Vec<PointingDevice> = con.query().unwrap_or_default();
    let mice = pointing_devices
        .iter()
        .filter(|d| d.pointing_type != Some(POINTING_TYPE_TOUCH_PAD))
        .filter_map(|d| d.description.clone())
        .map(|name| InputDeviceInfo { name })
        .collect();
    let touchpads = pointing_devices
        .into_iter()
        .filter(|d| d.pointing_type == Some(POINTING_TYPE_TOUCH_PAD))
        .filter_map(|d| d.description)
        .map(|name| InputDeviceInfo { name })
        .collect();

    let gamepads = con
        .query::<PnpEntity>()
        .map(|entities| {
            entities
                .into_iter()
                .filter_map(|e| e.name)
                .filter(|name| looks_like_gamepad(name))
                .map(|name| InputDeviceInfo { name })
                .collect()
        })
        .unwrap_or_default();

    InputDevices {
        mice,
        gamepads,
        touchpads,
    }
}
