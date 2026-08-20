use std::path::Path;
use tracker::remote_export::RemoteExportConfig;
use tracker::SystemReport;

/// Action demandée sur la config d'envoi distant via les arguments CLI.
/// `None` : ne pas toucher à `remote_export.json` (comportement historique).
enum RemoteAction {
    None,
    SetAndEnable { url: String, auth_token: Option<String> },
    Disable,
}

/// Parsing minimal, sans dépendance externe : `--remote-url <URL>`,
/// `--remote-token <TOKEN>` (doit accompagner `--remote-url`), `--remote-disable`.
fn parse_remote_action(args: &[String]) -> RemoteAction {
    let mut url: Option<String> = None;
    let mut auth_token: Option<String> = None;
    let mut disable = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--remote-url" => {
                let value = args.get(i + 1).unwrap_or_else(|| {
                    eprintln!("--remote-url nécessite une valeur (URL)");
                    std::process::exit(1);
                });
                url = Some(value.clone());
                i += 2;
            }
            "--remote-token" => {
                let value = args.get(i + 1).unwrap_or_else(|| {
                    eprintln!("--remote-token nécessite une valeur (TOKEN)");
                    std::process::exit(1);
                });
                auth_token = Some(value.clone());
                i += 2;
            }
            "--remote-disable" => {
                disable = true;
                i += 1;
            }
            other => {
                eprintln!("Argument inconnu : {other}");
                std::process::exit(1);
            }
        }
    }

    match (url, disable) {
        (Some(_), true) => {
            eprintln!("--remote-url et --remote-disable sont incompatibles");
            std::process::exit(1);
        }
        (Some(url), false) => RemoteAction::SetAndEnable { url, auth_token },
        (None, true) => RemoteAction::Disable,
        (None, false) => RemoteAction::None,
    }
}

fn main() {
    // Comportement historique du CLI : messages en français par défaut,
    // sauf si l'utilisateur exporte LANG=en... (aligné sur set_locale côté GUI).
    let locale = std::env::var("LANG")
        .ok()
        .and_then(|lang| lang.get(0..2).map(str::to_string))
        .filter(|code| code == "en")
        .unwrap_or_else(|| "fr".to_string());
    rust_i18n::set_locale(&locale);

    let args: Vec<String> = std::env::args().skip(1).collect();
    match parse_remote_action(&args) {
        RemoteAction::None => {}
        RemoteAction::SetAndEnable { url, auth_token } => {
            let config = RemoteExportConfig { enabled: true, url, auth_token };
            match tracker::remote_export::save(&config) {
                Ok(()) => println!("Configuration d'envoi distant enregistrée et activée"),
                Err(e) => eprintln!("Erreur lors de l'enregistrement de la config distante : {e}"),
            }
        }
        RemoteAction::Disable => {
            let mut config = tracker::remote_export::load().ok().flatten().unwrap_or_default();
            config.enabled = false;
            match tracker::remote_export::save(&config) {
                Ok(()) => println!("Envoi distant désactivé"),
                Err(e) => eprintln!("Erreur lors de la désactivation de l'envoi distant : {e}"),
            }
        }
    }

    let report = SystemReport::collect();

    match tracker::storage::record_snapshot(&report) {
        Ok(_) => println!("Snapshot enregistré dans l'historique local"),
        Err(e) => eprintln!("Erreur lors de l'enregistrement en base : {e}"),
    }

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

    let remote_config = tracker::remote_export::load().ok().flatten().unwrap_or_default();
    if remote_config.enabled {
        match report.to_json_pretty() {
            Ok(json_body) => match tracker::remote_export::send_report(&remote_config, &json_body) {
                Ok(()) => println!("Rapport envoyé au serveur distant ({})", remote_config.url),
                Err(e) => eprintln!("Erreur lors de l'envoi du rapport au serveur distant : {e}"),
            },
            Err(e) => {
                eprintln!("Erreur lors de la sérialisation du rapport pour l'export distant : {e}")
            }
        }
    }
}
