use super::{PartitionInfo, RaidArrayInfo, StorageLayoutInfo};

/// `diskutil list` (lecture libre) : parsing heuristique des lignes
/// "N: TYPE NAME SIZE_VALUE SIZE_UNIT IDENTIFIER" — on ne garde que les
/// identifiants de la forme "diskNsM" (partitions, pas les disques entiers).
fn collect_partitions() -> Vec<PartitionInfo> {
    let Some(text) = crate::command::run("diskutil", &["list"]) else {
        return Vec::new();
    };

    let mut partitions = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        let Some((idx, rest)) = trimmed.split_once(':') else { continue };
        if idx.parse::<u32>().is_err() {
            continue;
        }
        let tokens: Vec<&str> = rest.split_whitespace().collect();
        if tokens.len() < 3 {
            continue;
        }
        let identifier = tokens[tokens.len() - 1];
        if !identifier.contains('s') {
            continue;
        }
        let unit = tokens[tokens.len() - 2];
        let value = tokens[tokens.len() - 3].trim_start_matches('*');
        partitions.push(PartitionInfo {
            device: format!("/dev/{identifier}"),
            fs_type: tokens[0].to_string(),
            size_gb: to_gb(value, unit),
        });
    }
    partitions
}

fn to_gb(value: &str, unit: &str) -> u64 {
    let v: f64 = value.parse().unwrap_or(0.0);
    match unit {
        "TB" => (v * 1000.0) as u64,
        "GB" => v as u64,
        "MB" => (v / 1000.0) as u64,
        _ => 0,
    }
}

/// `diskutil appleRAID list` (lecture libre) affiche un bloc par ensemble
/// RAID logiciel Apple, séparé par des lignes de `=`. Absence de RAID
/// configuré (cas le plus fréquent) renvoie un message texte plutôt qu'un
/// tableau, traité ici comme "aucun tableau" (aucune ligne `Name:` trouvée).
fn collect_raid() -> Vec<RaidArrayInfo> {
    let Some(text) = crate::command::run("diskutil", &["appleRAID", "list"]) else {
        return Vec::new();
    };

    let mut arrays = Vec::new();
    let mut name: Option<String> = None;
    let mut level: Option<String> = None;
    let mut state: Option<String> = None;
    let mut devices: Vec<String> = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('=') {
            flush_raid_block(&mut name, &mut level, &mut state, &mut devices, &mut arrays);
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("Name:") {
            name = Some(rest.trim().to_string());
        } else if let Some(rest) = trimmed.strip_prefix("Type:") {
            level = Some(rest.trim().to_string());
        } else if let Some(rest) = trimmed.strip_prefix("Status:") {
            state = Some(rest.trim().to_string());
        } else if trimmed
            .split_whitespace()
            .next()
            .is_some_and(|first| first.parse::<u32>().is_ok())
        {
            if let Some(disk) = trimmed.split_whitespace().find(|token| token.starts_with("disk")) {
                devices.push(format!("/dev/{disk}"));
            }
        }
    }
    flush_raid_block(&mut name, &mut level, &mut state, &mut devices, &mut arrays);

    arrays
}

fn flush_raid_block(
    name: &mut Option<String>,
    level: &mut Option<String>,
    state: &mut Option<String>,
    devices: &mut Vec<String>,
    arrays: &mut Vec<RaidArrayInfo>,
) {
    if let Some(n) = name.take() {
        arrays.push(RaidArrayInfo {
            device: n,
            level: level.take().unwrap_or_else(|| "?".to_string()),
            state: state.take().unwrap_or_else(|| "?".to_string()),
            devices: std::mem::take(devices),
        });
    }
}

pub fn collect() -> StorageLayoutInfo {
    StorageLayoutInfo {
        partitions: collect_partitions(),
        lvm_volumes: Vec::new(),
        raid_arrays: collect_raid(),
    }
}
