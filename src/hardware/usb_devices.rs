#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct UsbDeviceInfo {
    pub name: String,
    pub vendor: Option<String>,
    /// Classification approximative (ex. "Mass Storage", "HID", "Hub",
    /// "Network") dérivée du code de classe USB déjà lu pour filtrer les
    /// hubs (Linux `bDeviceClass`, Windows `PNPClass`). `None` sur macOS
    /// (pas de source texte simple identifiée) ou si le code est inconnu.
    pub device_class: Option<String>,
}

/// Infaillible par design : absence de périphérique ou erreur d'accès
/// matériel renvoient simplement un `Vec` vide.
pub fn collect() -> Vec<UsbDeviceInfo> {
    crate::os_dispatch::dispatch_os!(linux::collect(), macos::collect(), windows::collect(), Vec::new())
}

/// Correspondance des codes de classe USB standards (`bDeviceClass`, voir
/// usb.org) vers un nom lisible. Utilisé par le backend Linux ; le
/// `PNPClass` Windows est une chaîne déjà lisible et n'a pas besoin de
/// cette table.
#[allow(dead_code)]
pub(crate) fn usb_class_name(code: &str) -> Option<&'static str> {
    Some(match code {
        "01" => "Audio",
        "02" => "Communications",
        "03" => "HID",
        "05" => "Physical",
        "06" => "Image",
        "07" => "Printer",
        "08" => "Mass Storage",
        "09" => "Hub",
        "0a" => "CDC-Data",
        "0b" => "Smart Card",
        "0d" => "Content Security",
        "0e" => "Video",
        "0f" => "Personal Healthcare",
        "10" => "Audio/Video",
        "11" => "Billboard",
        "dc" => "Diagnostic",
        "e0" => "Wireless Controller",
        "ef" => "Miscellaneous",
        "fe" => "Application Specific",
        "ff" => "Vendor Specific",
        _ => return None,
    })
}
