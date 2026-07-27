use super::UsbDeviceInfo;
use std::collections::HashMap;

/// Libellés génériques de bus/hub à exclure du résultat (pas de vrai
/// périphérique externe).
const GENERIC_LABELS: &[&str] = &["USB Bus", "Host Controller", "Hub"];

/// `ioreg -p IOUSB -l` (lecture libre) expose `bDeviceClass`/`bInterfaceClass`
/// en décimal dans les propriétés IOKit de chaque nœud USB — absents de
/// `system_profiler SPUSBDataType`. Association best-effort par nom de
/// produit (`"USB Product Name"`), le nom exact pouvant différer légèrement
/// de celui affiché par `system_profiler` selon les périphériques.
fn parse_ioreg_classes() -> HashMap<String, String> {
    let mut classes = HashMap::new();
    let Some(text) = crate::command::run("ioreg", &["-p", "IOUSB", "-l", "-w0"]) else {
        return classes;
    };

    let mut current_name: Option<String> = None;
    let mut current_class: Option<String> = None;

    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("+-o ") {
            if let (Some(name), Some(class)) = (current_name.take(), current_class.take()) {
                classes.entry(name).or_insert(class);
            }
            current_name = rest.split(['@', '<']).next().map(|s| s.trim().to_string());
            current_class = None;
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("\"USB Product Name\" = \"") {
            if let Some(end) = rest.find('"') {
                current_name = Some(rest[..end].to_string());
            }
        } else if let Some(rest) = trimmed.strip_prefix("\"bInterfaceClass\" = ") {
            if let Ok(code) = rest.trim().parse::<u32>() {
                current_class = super::usb_class_name(&format!("{code:02x}")).map(|s| s.to_string());
            }
        } else if current_class.is_none() {
            if let Some(rest) = trimmed.strip_prefix("\"bDeviceClass\" = ") {
                if let Ok(code) = rest.trim().parse::<u32>() {
                    if code != 0 {
                        current_class = super::usb_class_name(&format!("{code:02x}")).map(|s| s.to_string());
                    }
                }
            }
        }
    }
    if let (Some(name), Some(class)) = (current_name, current_class) {
        classes.entry(name).or_insert(class);
    }

    classes
}

pub fn collect() -> Vec<UsbDeviceInfo> {
    let Some(text) = crate::command::run("system_profiler", &["SPUSBDataType"]) else {
        return Vec::new();
    };
    let classes = parse_ioreg_classes();

    text.lines()
        .filter(|line| line.trim_end().ends_with(':') && !line.trim().is_empty())
        .map(|line| line.trim().trim_end_matches(':').to_string())
        .filter(|name| {
            !GENERIC_LABELS
                .iter()
                .any(|label| name.contains(label))
        })
        .map(|name| {
            let device_class = classes.get(&name).cloned();
            UsbDeviceInfo {
                name,
                vendor: None,
                device_class,
            }
        })
        .collect()
}
