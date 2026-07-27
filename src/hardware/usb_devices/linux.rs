use super::UsbDeviceInfo;
use std::fs;

/// Classe USB 09 = hub (voir usb.org device class definitions).
const USB_CLASS_HUB: &str = "09";
/// Classe "définie au niveau interface" (périphériques composites) : la
/// vraie classification se trouve alors sur les interfaces, pas sur le
/// device lui-même — cas fréquent (imprimantes multifonctions, clés
/// composites stockage+lecteur de carte, etc.).
const USB_CLASS_INTERFACE_DEFINED: &str = "00";

/// Cherche `bInterfaceClass` sur la première interface non-hub d'un
/// périphérique composite. Les répertoires d'interface (ex. "1-2:1.0")
/// apparaissent comme des entrées sœurs du device dans le même dossier
/// sysfs — lecture libre, aucune élévation requise.
fn read_interface_class(all_entries: &[String], device_name: &str) -> Option<String> {
    let prefix = format!("{device_name}:");
    all_entries.iter().filter(|name| name.starts_with(&prefix)).find_map(|iface| {
        let class = fs::read_to_string(format!("/sys/bus/usb/devices/{iface}/bInterfaceClass"))
            .ok()?
            .trim()
            .to_lowercase();
        if class == USB_CLASS_HUB {
            None
        } else {
            super::usb_class_name(&class).map(|s| s.to_string())
        }
    })
}

pub fn collect() -> Vec<UsbDeviceInfo> {
    let mut devices = Vec::new();
    let Ok(entries) = fs::read_dir("/sys/bus/usb/devices") else {
        return devices;
    };
    let all_entries: Vec<String> =
        entries.filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().to_string()).collect();

    for name in &all_entries {
        // Les entrées d'interface (ex. "1-2:1.0") sont traitées séparément
        // via `read_interface_class`, pas comme des devices à part entière.
        if name.contains(':') {
            continue;
        }
        // Les entrées "usbN" sont les hubs racine des contrôleurs, jamais de
        // vrais périphériques externes.
        if name.starts_with("usb") {
            continue;
        }

        let path = format!("/sys/bus/usb/devices/{name}");
        let class = fs::read_to_string(format!("{path}/bDeviceClass"))
            .ok()
            .map(|s| s.trim().to_lowercase());
        if class.as_deref() == Some(USB_CLASS_HUB) {
            continue;
        }

        let device_class = if class.is_none() || class.as_deref() == Some(USB_CLASS_INTERFACE_DEFINED) {
            read_interface_class(&all_entries, name)
        } else {
            class.as_deref().and_then(super::usb_class_name).map(|s| s.to_string())
        };

        let Ok(product) = fs::read_to_string(format!("{path}/product")) else {
            continue;
        };
        let product = product.trim().to_string();
        if product.is_empty() {
            continue;
        }

        let vendor = fs::read_to_string(format!("{path}/manufacturer"))
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        devices.push(UsbDeviceInfo {
            name: product,
            vendor,
            device_class,
        });
    }

    devices
}
