use super::{InputDeviceInfo, InputDevices};
use std::fs;

struct DeviceBlock {
    name: String,
    handlers: String,
}

/// /proc/bus/input/devices liste tous les périphériques d'entrée par blocs
/// séparés par une ligne vide, avec une ligne `N: Name="..."` et une ligne
/// `H: Handlers=...` par bloc.
fn parse_blocks(content: &str) -> Vec<DeviceBlock> {
    content
        .split("\n\n")
        .filter_map(|block| {
            let name = block
                .lines()
                .find_map(|line| line.strip_prefix("N: Name="))
                .map(|s| s.trim().trim_matches('"').to_string())?;
            let handlers = block
                .lines()
                .find_map(|line| line.strip_prefix("H: Handlers="))
                .unwrap_or("")
                .trim()
                .to_string();
            Some(DeviceBlock { name, handlers })
        })
        .collect()
}

pub fn collect() -> InputDevices {
    let Ok(content) = fs::read_to_string("/proc/bus/input/devices") else {
        return InputDevices {
            mice: Vec::new(),
            gamepads: Vec::new(),
            touchpads: Vec::new(),
        };
    };
    let blocks = parse_blocks(&content);

    // Un touchpad s'enregistre aussi comme périphérique "mouseN" dans
    // Handlers : le filtre doit donc porter sur le nom, pas sur le handler.
    let is_touchpad = |name: &str| {
        let lower = name.to_lowercase();
        lower.contains("touchpad") || lower.contains("trackpad")
    };

    let mice = blocks
        .iter()
        .filter(|b| b.name.to_lowercase().contains("mouse") && !is_touchpad(&b.name))
        .map(|b| InputDeviceInfo {
            name: b.name.clone(),
        })
        .collect();

    let touchpads = blocks
        .iter()
        .filter(|b| is_touchpad(&b.name))
        .map(|b| InputDeviceInfo {
            name: b.name.clone(),
        })
        .collect();

    // Le handler jsN (API joystick du noyau, module joydev) est un
    // discriminant plus fiable qu'un filtre sur le nom, très variable
    // d'une manette à l'autre.
    let gamepads = blocks
        .iter()
        .filter(|b| {
            b.handlers
                .split_whitespace()
                .any(|h| h.strip_prefix("js").is_some_and(|n| n.chars().all(|c| c.is_ascii_digit())))
        })
        .map(|b| InputDeviceInfo {
            name: b.name.clone(),
        })
        .collect();

    InputDevices {
        mice,
        gamepads,
        touchpads,
    }
}
