use super::PeripheralInfo;
use std::fs;
use std::process::Command;

/// Le pseudo-fichier /proc/bus/input/devices liste tous les périphériques
/// d'entrée par blocs séparés par une ligne vide, avec une ligne
/// `N: Name="..."` par bloc.
fn keyboards() -> Vec<PeripheralInfo> {
    let Ok(content) = fs::read_to_string("/proc/bus/input/devices") else {
        return Vec::new();
    };

    content
        .lines()
        .filter_map(|line| line.strip_prefix("N: Name="))
        .map(|name| name.trim().trim_matches('"').to_string())
        .filter(|name| name.to_lowercase().contains("keyboard"))
        .map(|name| PeripheralInfo {
            name,
            kind: "Clavier".to_string(),
        })
        .collect()
}

/// `pactl` (PulseAudio/PipeWire) donne une description lisible des sorties
/// audio ("enceintes"), plus fiable qu'une lecture directe d'ALSA.
fn speakers() -> Vec<PeripheralInfo> {
    // LC_ALL=C : force une sortie anglaise pour un parsing fiable quelle que
    // soit la locale du système (ex: "Description :" en français).
    let Ok(output) = Command::new("pactl")
        .args(["list", "sinks"])
        .env("LC_ALL", "C")
        .output()
    else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(&output.stdout);

    text.lines()
        .filter_map(|line| line.trim().strip_prefix("Description: "))
        .map(|name| PeripheralInfo {
            name: name.to_string(),
            kind: "Enceintes".to_string(),
        })
        .collect()
}

/// Comme `speakers()` mais nécessite un parsing par bloc `Source #N` : il
/// faut lire à la fois `Name:` (pour exclure les flux `.monitor`, qui ne
/// sont pas des micros mais une boucle de retour des sorties audio) et
/// `Description:` (pour l'affichage) au sein d'un même bloc.
fn microphones() -> Vec<PeripheralInfo> {
    let Ok(output) = Command::new("pactl")
        .args(["list", "sources"])
        .env("LC_ALL", "C")
        .output()
    else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(&output.stdout);

    let mut microphones = Vec::new();
    let mut current_is_monitor = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(name) = trimmed.strip_prefix("Name: ") {
            current_is_monitor = name.ends_with(".monitor");
        } else if let Some(description) = trimmed.strip_prefix("Description: ") {
            if !current_is_monitor {
                microphones.push(PeripheralInfo {
                    name: description.to_string(),
                    kind: "Microphone".to_string(),
                });
            }
        }
    }
    microphones
}

pub fn collect() -> Vec<PeripheralInfo> {
    let mut peripherals = keyboards();
    peripherals.extend(speakers());
    peripherals.extend(microphones());
    peripherals
}
