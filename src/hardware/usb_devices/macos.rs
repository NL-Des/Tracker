use super::UsbDeviceInfo;

/// Libellés génériques de bus/hub à exclure du résultat (pas de vrai
/// périphérique externe).
const GENERIC_LABELS: &[&str] = &["USB Bus", "Host Controller", "Hub"];

pub fn collect() -> Vec<UsbDeviceInfo> {
    let Some(text) = crate::command::run("system_profiler", &["SPUSBDataType"]) else {
        return Vec::new();
    };

    text.lines()
        .filter(|line| line.trim_end().ends_with(':') && !line.trim().is_empty())
        .map(|line| line.trim().trim_end_matches(':').to_string())
        .filter(|name| {
            !GENERIC_LABELS
                .iter()
                .any(|label| name.contains(label))
        })
        .map(|name| UsbDeviceInfo {
            name,
            vendor: None,
            device_class: None,
        })
        .collect()
}
