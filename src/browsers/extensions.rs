use super::BrowserExtensionInfo;
use std::path::Path;

/// Lit les extensions d'un navigateur Chromium (Chrome/Chromium/Brave/Edge/
/// Opera/Vivaldi) depuis le dossier de profil : `<profil>/Extensions/<id>/<version>/manifest.json`.
/// Lecture seule dans le profil de l'utilisateur courant, aucune élévation
/// requise. Le `name` peut être une clé de traduction (`__MSG_xxx__`) pour
/// les extensions localisées : on la garde telle quelle plutôt que de
/// résoudre les fichiers de locale, hors scope pour un inventaire.
pub fn read_chromium_extensions(profile_dir: &Path) -> Vec<BrowserExtensionInfo> {
    let extensions_dir = profile_dir.join("Extensions");
    let Ok(id_entries) = std::fs::read_dir(&extensions_dir) else {
        return Vec::new();
    };

    let mut extensions = Vec::new();
    for id_entry in id_entries.filter_map(|e| e.ok()) {
        let id = id_entry.file_name().to_string_lossy().to_string();
        // Chrome Web Store réserve cet ID pour ses propres composants internes.
        if id == "Temp" {
            continue;
        }
        let Ok(version_entries) = std::fs::read_dir(id_entry.path()) else {
            continue;
        };
        // Une extension peut avoir plusieurs dossiers de version après une
        // mise à jour : on prend le premier trouvé avec un manifest valide.
        for version_entry in version_entries.filter_map(|e| e.ok()) {
            let manifest_path = version_entry.path().join("manifest.json");
            let Ok(text) = std::fs::read_to_string(&manifest_path) else {
                continue;
            };
            let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else {
                continue;
            };
            let name = json
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or(&id)
                .to_string();
            let version = json
                .get("version")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| version_entry.file_name().to_string_lossy().to_string());
            extensions.push(BrowserExtensionInfo { id: id.clone(), name, version });
            break;
        }
    }
    extensions
}

/// Lit les extensions Firefox depuis `<profil>/extensions.json`. Lecture
/// seule, aucune élévation requise.
pub fn read_firefox_extensions(profile_dir: &Path) -> Vec<BrowserExtensionInfo> {
    let Ok(text) = std::fs::read_to_string(profile_dir.join("extensions.json")) else {
        return Vec::new();
    };
    let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else {
        return Vec::new();
    };
    let Some(addons) = json.get("addons").and_then(|v| v.as_array()) else {
        return Vec::new();
    };

    addons
        .iter()
        // Ne garder que les vraies extensions installées par l'utilisateur,
        // pas les composants internes du navigateur.
        .filter(|addon| addon.get("type").and_then(|v| v.as_str()) == Some("extension"))
        .filter_map(|addon| {
            let id = addon.get("id")?.as_str()?.to_string();
            let version = addon
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let name = addon
                .get("defaultLocale")
                .and_then(|l| l.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or(&id)
                .to_string();
            Some(BrowserExtensionInfo { id, name, version })
        })
        .collect()
}
