use super::UsbDeviceInfo;
use std::fs;

/// Classe USB 09 = hub (voir usb.org device class definitions).
const USB_CLASS_HUB: &str = "09";

pub fn collect() -> Vec<UsbDeviceInfo> {
    let mut devices = Vec::new();
    let Ok(entries) = fs::read_dir("/sys/bus/usb/devices") else {
        return devices;
    };

    for entry in entries.filter_map(|e| e.ok()) {
        let name = entry.file_name().to_string_lossy().to_string();
        // Les entrées "usbN" sont les hubs racine des contrôleurs, jamais de
        // vrais périphériques externes.
        if name.starts_with("usb") {
            continue;
        }

        let path = entry.path();
        let class = fs::read_to_string(path.join("bDeviceClass"))
            .ok()
            .map(|s| s.trim().to_lowercase());
        if class.as_deref() == Some(USB_CLASS_HUB) {
            continue;
        }
        let device_class = class
            .as_deref()
            .and_then(super::usb_class_name)
            .map(|s| s.to_string());

        let Ok(product) = fs::read_to_string(path.join("product")) else {
            continue;
        };
        let product = product.trim().to_string();
        if product.is_empty() {
            continue;
        }

        let vendor = fs::read_to_string(path.join("manufacturer"))
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
