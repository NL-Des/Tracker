mod browsers;
mod command;
mod display;
mod hardware;
mod markdown;
mod os_dispatch;
mod report;
mod software;

use report::SystemReport;
use std::path::Path;

fn main() {
    let report = SystemReport::collect();

    display::print_report(&report);

    let json_path = Path::new("tracker_report.json");
    match report.save_json(json_path) {
        Ok(()) => println!("\nRapport JSON exporté vers {}", json_path.display()),
        Err(e) => eprintln!("\nErreur lors de l'export JSON : {e}"),
    }

    let markdown_path = Path::new("tracker_report.md");
    match report.save_markdown(markdown_path) {
        Ok(()) => println!("Rapport Markdown exporté vers {}", markdown_path.display()),
        Err(e) => eprintln!("Erreur lors de l'export Markdown : {e}"),
    }
}
