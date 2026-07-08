use super::PeripheralInfo;
use serde::Deserialize;
use wmi::WMIConnection;

#[derive(Deserialize)]
#[serde(rename = "Win32_Keyboard")]
struct Keyboard {
    #[serde(rename = "Description")]
    description: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename = "Win32_SoundDevice")]
struct SoundDevice {
    #[serde(rename = "Name")]
    name: Option<String>,
}

pub fn collect() -> Vec<PeripheralInfo> {
    let mut peripherals = Vec::new();

    let Ok(con) = WMIConnection::new() else {
        return peripherals;
    };

    if let Ok(keyboards) = con.query::<Keyboard>() {
        for keyboard in keyboards {
            peripherals.push(PeripheralInfo {
                name: keyboard.description.unwrap_or_else(|| "Clavier".to_string()),
                kind: "Clavier".to_string(),
            });
        }
    }

    if let Ok(sound_devices) = con.query::<SoundDevice>() {
        for device in sound_devices {
            peripherals.push(PeripheralInfo {
                name: device.name.unwrap_or_else(|| "Périphérique audio".to_string()),
                kind: "Enceintes".to_string(),
            });
        }
    }

    peripherals
}
