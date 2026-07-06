mod browsers;
mod display;
mod hardware;
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
}
