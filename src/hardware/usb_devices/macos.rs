use super::UsbDeviceInfo;
use std::process::Command;

/// Libellés génériques de bus/hub à exclure du résultat (pas de vrai
/// périphérique externe).
const GENERIC_LABELS: &[&str] = &["USB Bus", "Host Controller", "Hub"];

pub fn collect() -> Vec<UsbDeviceInfo> {
    let Ok(output) = Command::new("system_profiler")
        .arg("SPUSBDataType")
        .output()
    else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(&output.stdout);

    text.lines()
        .filter(|line| line.trim_end().ends_with(':') && !line.trim().is_empty())
        .map(|line| line.trim().trim_end_matches(':').to_string())
        .filter(|name| {
            !GENERIC_LABELS
                .iter()
                .any(|label| name.contains(label))
        })
        .map(|name| UsbDeviceInfo { name, vendor: None })
        .collect()
}
