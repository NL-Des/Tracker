use crate::consent::{ConsentConfig, ConsentPreset};
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
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_cpu_all();

        let hardware = hardware::collect(&sys);
        let software = software::collect(&sys);
        let browsers = browsers::collect();

        let mut collection_warnings = Vec::new();
        if hardware.motherboard.machine_uuid.is_none() {
            collection_warnings.push(rust_i18n::t!("warnings.machine_uuid_unavailable").to_string());
        }
        if hardware.monitors.is_empty() {
            collection_warnings.push(rust_i18n::t!("warnings.no_monitor_detected").to_string());
        }
        if hardware.gpus.is_empty() {
            collection_warnings.push(rust_i18n::t!("warnings.no_gpu_detected").to_string());
        }
        if browsers.is_empty() {
            collection_warnings.push(rust_i18n::t!("warnings.no_browser_detected").to_string());
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

    /// Sérialise le rapport en remplaçant par `"np"` chaque champ Hardware/Software
    /// désactivé dans `consent` (cf. plan_client.md étape 9). Introspecte
    /// dynamiquement `consent.hardware`/`consent.software` plutôt que de coder
    /// les ~47 clés en dur, pour rester aligné avec `HARDWARE_FIELDS`/`SOFTWARE_FIELDS`.
    pub fn to_json_pretty_filtered(&self, consent: &ConsentConfig) -> serde_json::Result<String> {
        let mut value = serde_json::to_value(self)?;
        filter_module(&mut value, "hardware", &consent.hardware)?;
        filter_module(&mut value, "software", &consent.software)?;
        if !consent.browsers {
            if let Some(browsers) = value.get_mut("browsers") {
                *browsers = serde_json::Value::String("np".to_string());
            }
        }
        serde_json::to_string_pretty(&value)
    }

    pub fn save_json(&self, path: &Path) -> std::io::Result<()> {
        let json = self
            .to_json_pretty()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, json)
    }

    pub fn save_json_filtered(&self, path: &Path, consent: &ConsentConfig) -> std::io::Result<()> {
        let json = self
            .to_json_pretty_filtered(consent)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, json)
    }

    /// CLI (main.rs) : comportement historique inchangé, aucun filtrage.
    pub fn save_markdown(&self, path: &Path) -> std::io::Result<()> {
        std::fs::write(path, crate::markdown::generate(self, &ConsentPreset::Maximum.to_config()))
    }

    pub fn save_markdown_filtered(&self, path: &Path, consent: &ConsentConfig) -> std::io::Result<()> {
        std::fs::write(path, crate::markdown::generate(self, consent))
    }

    /// CLI (main.rs) : comportement historique inchangé, aucun filtrage.
    pub fn save_xml(&self, path: &Path) -> std::io::Result<()> {
        std::fs::write(path, crate::xml::generate(self, &ConsentPreset::Maximum.to_config()))
    }

    pub fn save_xml_filtered(&self, path: &Path, consent: &ConsentConfig) -> std::io::Result<()> {
        std::fs::write(path, crate::xml::generate(self, consent))
    }
}

/// Remplace par `"np"` chaque clé de `value[module]` dont le booléen correspondant
/// dans `consent_module` (`HardwareConsent`/`SoftwareConsent`) est `false`.
fn filter_module<T: Serialize>(
    value: &mut serde_json::Value,
    module: &str,
    consent_module: &T,
) -> serde_json::Result<()> {
    let consent_value = serde_json::to_value(consent_module)?;
    let consent_object = consent_value
        .as_object()
        .expect("HardwareConsent/SoftwareConsent doivent toujours sérialiser en objet JSON");
    if let Some(module_object) = value.get_mut(module).and_then(|v| v.as_object_mut()) {
        for (key, enabled) in consent_object {
            if enabled.as_bool() == Some(false) {
                module_object.insert(key.clone(), serde_json::Value::String("np".to_string()));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod i18n_tests {
    // `rust_i18n::set_locale` mute un état global au processus : un mutex évite
    // que ces tests ne se marchent dessus s'ils tournent sur des threads distincts.
    static LOCALE_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn warning_translates_to_french() {
        let _guard = LOCALE_TEST_LOCK.lock().unwrap();
        rust_i18n::set_locale("fr");
        assert_eq!(
            rust_i18n::t!("warnings.no_gpu_detected"),
            "Aucun GPU détecté"
        );
    }

    #[test]
    fn warning_translates_to_english() {
        let _guard = LOCALE_TEST_LOCK.lock().unwrap();
        rust_i18n::set_locale("en");
        assert_eq!(rust_i18n::t!("warnings.no_gpu_detected"), "No GPU detected");
    }

    #[test]
    fn missing_key_falls_back_to_default_locale() {
        let _guard = LOCALE_TEST_LOCK.lock().unwrap();
        rust_i18n::set_locale("de");
        // Aucune locale "de" définie : rust-i18n retombe sur le fallback ("en").
        assert_eq!(rust_i18n::t!("warnings.no_gpu_detected"), "No GPU detected");
    }
}
