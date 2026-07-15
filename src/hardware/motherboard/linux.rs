use super::MotherboardInfo;
use std::fs;

/// `product_uuid` n'est souvent lisible qu'en root : un échec de lecture est
/// traité comme une simple absence de donnée, pas une erreur.
fn read_dmi_field(field: &str) -> Option<String> {
    let path = format!("/sys/class/dmi/id/{field}");
    fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

pub fn collect() -> MotherboardInfo {
    MotherboardInfo {
        vendor: read_dmi_field("board_vendor"),
        model: read_dmi_field("board_name"),
        version: read_dmi_field("board_version"),
        serial_number: read_dmi_field("board_serial"),
        bios_vendor: read_dmi_field("bios_vendor"),
        bios_version: read_dmi_field("bios_version"),
        bios_date: read_dmi_field("bios_date"),
        machine_uuid: read_dmi_field("product_uuid"),
        secure_boot: None,
        tpm_version: None,
    }
}
