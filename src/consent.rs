use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const CONSENT_SCHEMA_VERSION: u32 = 1;
const CONSENT_FILE_NAME: &str = "consent.json";

// ⚠️ Les noms de champs doivent rester identiques à ceux de `hardware::HardwareInfo`
// (src/hardware/mod.rs) — voir `tests/consent_parity.rs` qui vérifie cet alignement.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct HardwareConsent {
    pub cpu: bool,
    pub memory: bool,
    pub disks: bool,
    pub virtual_disks: bool,
    pub network: bool,
    pub wifi: bool,
    pub components: bool,
    pub batteries: bool,
    pub motherboard: bool,
    pub gpus: bool,
    pub pci_devices: bool,
    pub monitors: bool,
    pub optical_drives: bool,
    pub peripherals: bool,
    pub mice: bool,
    pub gamepads: bool,
    pub touchpads: bool,
    pub cameras: bool,
    pub usb_devices: bool,
    pub bluetooth_devices: bool,
    pub printers: bool,
    pub fans: bool,
    pub storage_layout: bool,
    pub power_profile: bool,
}

impl HardwareConsent {
    fn all(value: bool) -> Self {
        Self {
            cpu: value,
            memory: value,
            disks: value,
            virtual_disks: value,
            network: value,
            wifi: value,
            components: value,
            batteries: value,
            motherboard: value,
            gpus: value,
            pci_devices: value,
            monitors: value,
            optical_drives: value,
            peripherals: value,
            mice: value,
            gamepads: value,
            touchpads: value,
            cameras: value,
            usb_devices: value,
            bluetooth_devices: value,
            printers: value,
            fans: value,
            storage_layout: value,
            power_profile: value,
        }
    }
}

impl Default for HardwareConsent {
    fn default() -> Self {
        Self::all(false)
    }
}

// ⚠️ Les noms de champs doivent rester identiques à ceux de `software::SoftwareInfo`
// (src/software/mod.rs) — voir `tests/consent_parity.rs` qui vérifie cet alignement.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SoftwareConsent {
    pub os: bool,
    pub processes: bool,
    pub users: bool,
    pub env_vars: bool,
    pub installed_apps: bool,
    pub dev_runtimes: bool,
    pub services: bool,
    pub failed_services: bool,
    pub scheduled_tasks: bool,
    pub autostart_entries: bool,
    pub package_managers: bool,
    pub network_connections: bool,
    pub desktop_environment: bool,
    pub update_history: bool,
    pub kernel_modules: bool,
    pub docker_images: bool,
    pub docker_volumes: bool,
    pub virtual_machines: bool,
    pub podman_images: bool,
    pub podman_volumes: bool,
    pub fonts: bool,
    pub proxy_config: bool,
    pub ssh_keys: bool,
    pub security_status: bool,
}

impl SoftwareConsent {
    fn all(value: bool) -> Self {
        Self {
            os: value,
            processes: value,
            users: value,
            env_vars: value,
            installed_apps: value,
            dev_runtimes: value,
            services: value,
            failed_services: value,
            scheduled_tasks: value,
            autostart_entries: value,
            package_managers: value,
            network_connections: value,
            desktop_environment: value,
            update_history: value,
            kernel_modules: value,
            docker_images: value,
            docker_volumes: value,
            virtual_machines: value,
            podman_images: value,
            podman_volumes: value,
            fonts: value,
            proxy_config: value,
            ssh_keys: value,
            security_status: value,
        }
    }
}

impl Default for SoftwareConsent {
    fn default() -> Self {
        Self::all(false)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ConsentConfig {
    pub version: u32,
    pub accepted_at_unix: Option<u64>,
    pub hardware: HardwareConsent,
    pub software: SoftwareConsent,
    pub browsers: bool,
}

impl Default for ConsentConfig {
    fn default() -> Self {
        Self {
            version: CONSENT_SCHEMA_VERSION,
            accepted_at_unix: None,
            hardware: HardwareConsent::default(),
            software: SoftwareConsent::default(),
            browsers: false,
        }
    }
}

/// Niveaux globaux prédéfinis, voir bilan_client.md §5 étape 5.
/// La composition exacte de `Minimum`/`Medium` reste à valider avec l'utilisateur.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConsentPreset {
    None,
    Minimum,
    Medium,
    Maximum,
}

impl ConsentPreset {
    pub fn to_config(self) -> ConsentConfig {
        let (hardware, software, browsers) = match self {
            ConsentPreset::None => (HardwareConsent::all(false), SoftwareConsent::all(false), false),
            ConsentPreset::Minimum => {
                let mut hw = HardwareConsent::all(false);
                hw.cpu = true;
                hw.memory = true;
                let mut sw = SoftwareConsent::all(false);
                sw.os = true;
                (hw, sw, false)
            }
            ConsentPreset::Medium => (HardwareConsent::all(true), SoftwareConsent::all(false), false),
            ConsentPreset::Maximum => (HardwareConsent::all(true), SoftwareConsent::all(true), true),
        };
        ConsentConfig {
            version: CONSENT_SCHEMA_VERSION,
            accepted_at_unix: None,
            hardware,
            software,
            browsers,
        }
    }
}

fn config_path_in(base_dir: &Path) -> PathBuf {
    base_dir.join(CONSENT_FILE_NAME)
}

pub fn config_dir() -> std::io::Result<PathBuf> {
    directories::ProjectDirs::from("com", "tracker", "tracker")
        .map(|dirs| dirs.config_dir().to_path_buf())
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "répertoire de configuration utilisateur introuvable",
            )
        })
}

pub fn config_path() -> std::io::Result<PathBuf> {
    Ok(config_path_in(&config_dir()?))
}

fn load_from(path: &Path) -> std::io::Result<Option<ConsentConfig>> {
    if !path.exists() {
        return Ok(None);
    }
    let content = std::fs::read_to_string(path)?;
    let config = serde_json::from_str(&content)?;
    Ok(Some(config))
}

fn save_to(path: &Path, config: &ConsentConfig) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(config)?;
    std::fs::write(path, content)
}

pub fn load() -> std::io::Result<Option<ConsentConfig>> {
    load_from(&config_path()?)
}

pub fn save(config: &ConsentConfig) -> std::io::Result<()> {
    save_to(&config_path()?, config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_serializes_expected_shape() {
        let config = ConsentConfig::default();
        let value = serde_json::to_value(&config).unwrap();
        assert_eq!(value["version"], CONSENT_SCHEMA_VERSION);
        assert!(value["hardware"]["cpu"].is_boolean());
        assert!(value["software"]["os"].is_boolean());
    }

    #[test]
    fn maximum_preset_enables_everything() {
        let config = ConsentPreset::Maximum.to_config();
        assert_eq!(config.hardware, HardwareConsent::all(true));
        assert_eq!(config.software, SoftwareConsent::all(true));
        assert!(config.browsers);
    }

    #[test]
    fn none_preset_disables_everything() {
        let config = ConsentPreset::None.to_config();
        assert_eq!(config, ConsentConfig::default());
    }

    #[test]
    fn save_and_load_round_trip_via_temp_dir() {
        let dir = std::env::temp_dir().join(format!("tracker-consent-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = config_path_in(&dir);

        let config = ConsentPreset::Maximum.to_config();
        save_to(&path, &config).unwrap();
        let loaded = load_from(&path).unwrap().expect("le fichier vient d'être créé");
        assert_eq!(loaded, config);

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn load_from_missing_path_returns_none() {
        let dir = std::env::temp_dir().join(format!("tracker-consent-missing-{}", std::process::id()));
        let path = config_path_in(&dir);
        assert_eq!(load_from(&path).unwrap(), None);
    }
}
