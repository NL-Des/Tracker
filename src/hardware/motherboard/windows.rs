use super::MotherboardInfo;
use serde::Deserialize;
use wmi::WMIConnection;

#[derive(Deserialize)]
#[serde(rename = "Win32_BaseBoard")]
struct BaseBoard {
    #[serde(rename = "Manufacturer")]
    manufacturer: Option<String>,
    #[serde(rename = "Product")]
    product: Option<String>,
    #[serde(rename = "Version")]
    version: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename = "Win32_BIOS")]
struct Bios {
    #[serde(rename = "Manufacturer")]
    manufacturer: Option<String>,
    #[serde(rename = "SMBIOSBIOSVersion")]
    smbios_bios_version: Option<String>,
    #[serde(rename = "ReleaseDate")]
    release_date: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename = "Win32_ComputerSystemProduct")]
struct ComputerSystemProduct {
    #[serde(rename = "UUID")]
    uuid: Option<String>,
}

/// Infaillible par design : une erreur de connexion WMI ou une requête vide
/// laisse simplement les champs correspondants à `None`.
pub fn collect() -> MotherboardInfo {
    let mut info = MotherboardInfo::default();

    let Ok(con) = WMIConnection::new() else {
        return info;
    };

    if let Ok(mut boards) = con.query::<BaseBoard>() {
        if let Some(board) = boards.pop() {
            info.vendor = board.manufacturer;
            info.model = board.product;
            info.version = board.version;
        }
    }

    if let Ok(mut bios_list) = con.query::<Bios>() {
        if let Some(bios) = bios_list.pop() {
            info.bios_vendor = bios.manufacturer;
            info.bios_version = bios.smbios_bios_version;
            info.bios_date = bios.release_date;
        }
    }

    if let Ok(mut products) = con.query::<ComputerSystemProduct>() {
        if let Some(product) = products.pop() {
            info.machine_uuid = product.uuid;
        }
    }

    info
}
