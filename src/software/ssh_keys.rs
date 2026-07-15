use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct SshKeyInfo {
    pub file_name: String,
    pub key_type: Option<String>,
    pub fingerprint: Option<String>,
}

fn ssh_dir() -> Option<PathBuf> {
    let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).ok()?;
    Some(PathBuf::from(home).join(".ssh"))
}

/// Ne scanne que les clés publiques (`*.pub`) : jamais le contenu d'une clé
/// privée. Le dossier `~/.ssh` existe de la même façon sur les 3 OS, donc
/// pas de sous-module OS-specific ici. Infaillible par design : dossier
/// absent ou vide renvoie un `Vec` vide.
pub fn collect() -> Vec<SshKeyInfo> {
    let Some(dir) = ssh_dir() else {
        return Vec::new();
    };
    let Ok(entries) = fs::read_dir(&dir) else {
        return Vec::new();
    };

    entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("pub"))
        .map(|entry| {
            let path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();
            let key_type = fs::read_to_string(&path)
                .ok()
                .and_then(|content| content.split_whitespace().next().map(|s| s.to_string()));
            let fingerprint = crate::command::run("ssh-keygen", &["-lf", &path.to_string_lossy()])
                .and_then(|out| out.split_whitespace().nth(1).map(|s| s.to_string()));
            SshKeyInfo {
                file_name,
                key_type,
                fingerprint,
            }
        })
        .collect()
}
