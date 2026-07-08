use super::PrinterInfo;
use serde::Deserialize;
use wmi::WMIConnection;

#[derive(Deserialize)]
#[serde(rename = "Win32_Printer")]
struct Printer {
    #[serde(rename = "Name")]
    name: Option<String>,
}

/// Pas d'équivalent WMI simple pour les scanners (API WIA non exposée sans
/// dépendance supplémentaire) : seules les imprimantes sont détectées.
pub fn collect() -> Vec<PrinterInfo> {
    let Ok(con) = WMIConnection::new() else {
        return Vec::new();
    };
    let Ok(printers) = con.query::<Printer>() else {
        return Vec::new();
    };

    printers
        .into_iter()
        .filter_map(|p| p.name)
        .map(|name| PrinterInfo {
            name,
            kind: "Imprimante".to_string(),
        })
        .collect()
}
