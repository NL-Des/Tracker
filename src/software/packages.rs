use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
pub struct PackageManagerInfo {
    pub manager: String,
    pub package_count: usize,
}

/// (nom affiché, binaire, arguments listant les paquets). Un décompte de
/// lignes plutôt que le détail de chaque paquet : évite un rapport énorme,
/// la liste complète est déjà consultable via le gestionnaire lui-même.
const PACKAGE_MANAGERS: &[(&str, &str, &[&str])] = &[
    ("dpkg (apt)", "dpkg-query", &["-f", ".\n", "-W"]),
    ("rpm", "rpm", &["-qa"]),
    ("snap", "snap", &["list"]),
    ("flatpak", "flatpak", &["list", "--columns=application"]),
    ("cargo", "cargo", &["install", "--list"]),
    ("npm (global)", "npm", &["ls", "-g", "--depth=0"]),
    ("brew", "brew", &["list"]),
];

/// Infaillible par design : un gestionnaire absent ou une commande en échec
/// signifie simplement qu'il n'est pas utilisé sur la machine.
pub fn collect() -> Vec<PackageManagerInfo> {
    PACKAGE_MANAGERS
        .iter()
        .filter_map(|(manager, binary, args)| {
            let output = Command::new(binary).args(*args).output().ok()?;
            if !output.status.success() {
                return None;
            }
            let package_count = String::from_utf8_lossy(&output.stdout)
                .lines()
                .filter(|line| !line.trim().is_empty())
                .count();
            Some(PackageManagerInfo {
                manager: manager.to_string(),
                package_count,
            })
        })
        .collect()
}
