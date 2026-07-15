use super::{PartitionInfo, StorageLayoutInfo};

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

/// Pas d'équivalent LVM/RAID logiciel natif sur macOS pour ce périmètre.
pub fn collect() -> StorageLayoutInfo {
    StorageLayoutInfo {
        partitions: collect_partitions(),
        lvm_volumes: Vec::new(),
        raid_arrays: Vec::new(),
    }
}
