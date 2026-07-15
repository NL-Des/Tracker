use super::edid::EdidIds;
use serde::Deserialize;
use wmi::{COMLibrary, WMIConnection};

#[derive(Deserialize)]
#[serde(rename = "WmiMonitorID")]
struct WmiMonitorId {
    #[serde(rename = "ManufacturerName")]
    manufacturer_name: Option<Vec<u16>>,
    #[serde(rename = "ProductCodeID")]
    product_code_id: Option<Vec<u16>>,
    #[serde(rename = "SerialNumberID")]
    serial_number_id: Option<Vec<u16>>,
}

/// Les champs `WmiMonitorID` sont des tableaux de codes de caractères
/// (souvent zéro-terminés) plutôt que de vraies chaînes UTF-16.
fn decode_char_array(codes: &[u16]) -> Option<String> {
    let text: String = codes
        .iter()
        .take_while(|&&c| c != 0)
        .filter_map(|&c| char::from_u32(c as u32))
        .collect();
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

/// `WmiMonitorID` vit dans le namespace `ROOT\WMI` (différent du namespace
/// par défaut `ROOT\CIMV2` utilisé ailleurs dans le projet). Aucune
/// élévation requise pour cette lecture.
pub fn read_all() -> Vec<EdidIds> {
    let Ok(com_con) = COMLibrary::new() else {
        return Vec::new();
    };
    let Ok(con) = WMIConnection::with_namespace_path("ROOT\\WMI", com_con) else {
        return Vec::new();
    };
    let Ok(monitors) = con.query::<WmiMonitorId>() else {
        return Vec::new();
    };

    monitors
        .into_iter()
        .map(|m| EdidIds {
            vendor: m.manufacturer_name.as_deref().and_then(decode_char_array),
            model: m.product_code_id.as_deref().and_then(decode_char_array),
            serial_number: m.serial_number_id.as_deref().and_then(decode_char_array),
        })
        .collect()
}
