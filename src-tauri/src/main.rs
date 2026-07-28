#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::set_locale,
            commands::get_consent,
            commands::save_consent,
            commands::get_preset,
            commands::list_hardware_fields,
            commands::list_software_fields,
            commands::get_remote_export_config,
            commands::save_remote_export_config,
            commands::collect_and_export,
            commands::list_snapshots,
            commands::get_snapshot,
            commands::open_storage_location,
        ])
        .run(tauri::generate_context!())
        .expect("erreur lors du lancement de l'application tauri");
}
