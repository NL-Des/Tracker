use std::path::Path;
use tracker::SystemReport;

fn main() {
    // Comportement historique du CLI : messages en français par défaut,
    // sauf si l'utilisateur exporte LANG=en... (aligné sur set_locale côté GUI).
    let locale = std::env::var("LANG")
        .ok()
        .and_then(|lang| lang.get(0..2).map(str::to_string))
        .filter(|code| code == "en")
        .unwrap_or_else(|| "fr".to_string());
    rust_i18n::set_locale(&locale);

    let report = SystemReport::collect();

    let json_path = Path::new("tracker_report.json");
    match report.save_json(json_path) {
        Ok(()) => println!("Rapport JSON exporté vers {}", json_path.display()),
        Err(e) => eprintln!("Erreur lors de l'export JSON : {e}"),
    }

    let markdown_path = Path::new("tracker_report.md");
    match report.save_markdown(markdown_path) {
        Ok(()) => println!("Rapport Markdown exporté vers {}", markdown_path.display()),
        Err(e) => eprintln!("Erreur lors de l'export Markdown : {e}"),
    }

    let xml_path = Path::new("tracker_report.xml");
    match report.save_xml(xml_path) {
        Ok(()) => println!("Rapport XML exporté vers {}", xml_path.display()),
        Err(e) => eprintln!("Erreur lors de l'export XML : {e}"),
    }
}
