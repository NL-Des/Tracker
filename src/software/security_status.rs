use serde::Serialize;

#[derive(Serialize, Default)]
pub struct SecurityStatusInfo {
    /// État du pare-feu. Lecture libre sur Windows/macOS ; sur Linux, `ufw`
    /// nécessite généralement root sur la plupart des distributions, donc
    /// `None` la plupart du temps (tenté quand même, best-effort).
    pub firewall_enabled: Option<bool>,
    /// Statut de chiffrement du disque système (texte libre selon l'outil :
    /// "FileVault is On", "Encrypted"/"crypto_LUKS détecté", etc.).
    pub disk_encryption_status: Option<String>,
    /// Windows uniquement, via WMI `SecurityCenter2` (pas d'admin requis).
    /// Pas de concept standard équivalent non privilégié sur Linux/macOS.
    pub antivirus_product: Option<String>,
}

#[cfg(target_os = "linux")]
fn read_firewall_enabled() -> Option<bool> {
    // `ufw status` nécessite root sur la plupart des distributions : un échec
    // est traité comme une simple absence de donnée, pas une erreur.
    let text = crate::command::run("ufw", &["status"])?;
    let first_line = text.lines().next()?.to_lowercase();
    if first_line.contains("active") {
        Some(true)
    } else if first_line.contains("inactive") {
        Some(false)
    } else {
        None
    }
}

#[cfg(target_os = "macos")]
fn read_firewall_enabled() -> Option<bool> {
    let text = crate::command::run(
        "/usr/libexec/ApplicationFirewall/socketfilterfw",
        &["--getglobalstate"],
    )?;
    Some(text.to_lowercase().contains("enabled"))
}

#[cfg(target_os = "windows")]
fn read_firewall_enabled() -> Option<bool> {
    let text = crate::command::run("netsh", &["advfirewall", "show", "allprofiles", "state"])?;
    // Chaque profil affiche une ligne "State ON"/"State OFF" : le pare-feu
    // est considéré actif si au moins un profil l'est.
    Some(text.lines().any(|line| line.trim().eq_ignore_ascii_case("State ON")))
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn read_firewall_enabled() -> Option<bool> {
    None
}

#[cfg(target_os = "linux")]
fn read_disk_encryption_status() -> Option<String> {
    // Lecture libre, pas root requis pour `lsblk`.
    let text = crate::command::run("lsblk", &["-o", "NAME,FSTYPE"])?;
    if text.lines().any(|line| line.contains("crypto_LUKS")) {
        Some("crypto_LUKS détecté sur au moins une partition".to_string())
    } else {
        Some("Aucun volume LUKS détecté".to_string())
    }
}

#[cfg(target_os = "macos")]
fn read_disk_encryption_status() -> Option<String> {
    let text = crate::command::run("fdesetup", &["status"])?;
    let trimmed = text.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

/// `manage-bde`/`Get-BitLockerVolume` exigent généralement une élévation en
/// pratique malgré la documentation Microsoft : pas de source fiable non
/// privilégiée identifiée, `None` plutôt que de forcer une commande qui
/// échouerait silencieusement en environnement non admin.
#[cfg(target_os = "windows")]
fn read_disk_encryption_status() -> Option<String> {
    None
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn read_disk_encryption_status() -> Option<String> {
    None
}

#[derive(serde::Deserialize)]
#[cfg(target_os = "windows")]
#[serde(rename = "AntiVirusProduct")]
struct WmiAntiVirusProduct {
    #[serde(rename = "displayName")]
    display_name: Option<String>,
}

#[cfg(target_os = "windows")]
fn read_antivirus_product() -> Option<String> {
    let com_con = wmi::COMLibrary::new().ok()?;
    let con =
        wmi::WMIConnection::with_namespace_path("ROOT\\SecurityCenter2", com_con).ok()?;
    let mut products: Vec<WmiAntiVirusProduct> = con.query().ok()?;
    products.pop()?.display_name
}

#[cfg(not(target_os = "windows"))]
fn read_antivirus_product() -> Option<String> {
    None
}

/// Infaillible par design : chaque champ dégrade indépendamment vers `None`
/// selon la plateforme/les permissions disponibles.
pub fn collect() -> SecurityStatusInfo {
    SecurityStatusInfo {
        firewall_enabled: read_firewall_enabled(),
        disk_encryption_status: read_disk_encryption_status(),
        antivirus_product: read_antivirus_product(),
    }
}
