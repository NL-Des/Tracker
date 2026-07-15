#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize, Default)]
pub struct MotherboardInfo {
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub version: Option<String>,
    /// Numéro de série de la carte mère. Sur Linux, souvent inaccessible
    /// sans root selon la distribution (`board_serial` restreint) — un échec
    /// de lecture est traité comme une simple absence de donnée.
    pub serial_number: Option<String>,
    pub bios_vendor: Option<String>,
    pub bios_version: Option<String>,
    pub bios_date: Option<String>,
    pub machine_uuid: Option<String>,
    /// État Secure Boot ("enabled"/"disabled"), via `mokutil --sb-state`.
    /// Lecture libre sur la plupart des distributions Linux, aucune
    /// élévation requise.
    pub secure_boot: Option<String>,
    /// Version de spécification TPM ("1.2"/"2.0"), lecture libre via sysfs
    /// sur Linux ou WMI sur Windows ; pas d'équivalent standard sur macOS.
    pub tpm_version: Option<String>,
}

#[cfg(target_os = "linux")]
fn read_secure_boot_state() -> Option<String> {
    // Statut de sortie non vérifié ici (comportement existant conservé).
    let text = crate::command::run_lenient("mokutil", &["--sb-state"])?;
    text.lines().next().map(|line| line.trim().to_string())
}

#[cfg(not(target_os = "linux"))]
fn read_secure_boot_state() -> Option<String> {
    None
}

/// Version de spécification TPM ("1.2"/"2.0"), lecture libre via sysfs.
#[cfg(target_os = "linux")]
fn read_tpm_version() -> Option<String> {
    std::fs::read_to_string("/sys/class/tpm/tpm0/tpm_version_major")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

#[derive(serde::Deserialize)]
#[cfg(target_os = "windows")]
#[serde(rename = "Win32_Tpm")]
struct WmiTpm {
    #[serde(rename = "SpecVersion")]
    spec_version: Option<String>,
}

/// Version de spécification TPM via WMI, namespace `ROOT\CIMV2\Security\MicrosoftTpm`
/// (différent du namespace par défaut utilisé par les autres requêtes WMI du
/// projet). Aucune élévation requise pour cette lecture.
#[cfg(target_os = "windows")]
fn read_tpm_version() -> Option<String> {
    let com_con = wmi::COMLibrary::new().ok()?;
    let con =
        wmi::WMIConnection::with_namespace_path("ROOT\\CIMV2\\Security\\MicrosoftTpm", com_con)
            .ok()?;
    let mut tpms: Vec<WmiTpm> = con.query().ok()?;
    tpms.pop()?.spec_version
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn read_tpm_version() -> Option<String> {
    None
}

pub fn collect() -> MotherboardInfo {
    let mut info = crate::os_dispatch::dispatch_os!(
        linux::collect(),
        macos::collect(),
        windows::collect(),
        MotherboardInfo::default()
    );
    info.secure_boot = read_secure_boot_state();
    info.tpm_version = read_tpm_version();
    info
}
