use crate::{browsers, hardware, software};
use serde::Serialize;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::System;

#[derive(Serialize)]
pub struct SystemReport {
    pub generated_at_unix: u64,
    pub tool_version: String,
    pub hardware: hardware::HardwareInfo,
    pub software: software::SoftwareInfo,
    pub browsers: Vec<browsers::BrowserInfo>,
    pub collection_warnings: Vec<String>,
}

impl SystemReport {
    pub fn collect() -> SystemReport {
        let mut sys = System::new_all();
        sys.refresh_all();
        // Un deuxième rafraîchissement CPU est nécessaire car sysinfo a besoin
        // de deux mesures espacées dans le temps pour calculer un usage CPU fiable.
        sys.refresh_cpu_all();

        let hardware = hardware::collect(&sys);
        let software = software::collect(&sys);
        let browsers = browsers::collect();

        let mut collection_warnings = Vec::new();
        if hardware.motherboard.machine_uuid.is_none() {
            collection_warnings.push(
                "UUID machine inaccessible (souvent nécessite des privilèges root/admin)"
                    .to_string(),
            );
        }
        if hardware.monitors.is_empty() {
            collection_warnings
                .push("Aucun écran détecté (environnement headless/SSH ?)".to_string());
        }
        if hardware.gpus.is_empty() {
            collection_warnings.push("Aucun GPU détecté".to_string());
        }
        if browsers.is_empty() {
            collection_warnings.push("Aucun navigateur détecté".to_string());
        }

        SystemReport {
            generated_at_unix: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            tool_version: env!("CARGO_PKG_VERSION").to_string(),
            hardware,
            software,
            browsers,
            collection_warnings,
        }
    }

    pub fn to_json_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    pub fn save_json(&self, path: &Path) -> std::io::Result<()> {
        let json = self
            .to_json_pretty()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, json)
    }
}
