use std::path::Path;
use tracker::SystemReport;

fn main() {
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
