use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
pub struct DevRuntimeInfo {
    pub name: String,
    pub version: String,
}

/// (binaire, arguments pour obtenir la version). Liste volontairement courte
/// et non exhaustive : quelques runtimes de développement courants.
const RUNTIMES: &[(&str, &str, &[&str])] = &[
    ("Python", "python3", &["--version"]),
    ("Node.js", "node", &["--version"]),
    ("Java", "java", &["--version"]),
    ("Rust (rustc)", "rustc", &["--version"]),
    ("Go", "go", &["version"]),
    ("Ruby", "ruby", &["--version"]),
    ("PHP", "php", &["--version"]),
    (".NET", "dotnet", &["--version"]),
];

/// Infaillible par design : un binaire absent ou une erreur d'exécution
/// signifie simplement que le runtime n'est pas détecté.
pub fn collect() -> Vec<DevRuntimeInfo> {
    RUNTIMES
        .iter()
        .filter_map(|(name, binary, args)| {
            let output = Command::new(binary).args(*args).output().ok()?;
            if !output.status.success() {
                return None;
            }
            // `java --version`/`--version` écrivent parfois sur stderr selon la version.
            let text = if output.stdout.is_empty() {
                String::from_utf8_lossy(&output.stderr).to_string()
            } else {
                String::from_utf8_lossy(&output.stdout).to_string()
            };
            let version = text.lines().next()?.trim().to_string();
            Some(DevRuntimeInfo {
                name: name.to_string(),
                version,
            })
        })
        .collect()
}
