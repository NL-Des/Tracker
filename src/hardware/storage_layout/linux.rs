use super::{LvmVolumeInfo, PartitionInfo, RaidArrayInfo, StorageLayoutInfo};
use serde_json::Value;
use std::fs;

/// `lsblk -J -b` liste tous les blocs (disques + partitions) en JSON, lecture
/// libre. On ne garde que les nœuds `type == "part"`.
fn collect_partitions() -> Vec<PartitionInfo> {
    let Some(text) = crate::command::run("lsblk", &["-J", "-b", "-o", "NAME,TYPE,FSTYPE,SIZE"]) else {
        return Vec::new();
    };
    let Ok(root) = serde_json::from_str::<Value>(&text) else {
        return Vec::new();
    };

    let mut partitions = Vec::new();
    if let Some(devices) = root.get("blockdevices").and_then(|v| v.as_array()) {
        for device in devices {
            walk(device, &mut partitions);
        }
    }
    partitions
}

fn walk(node: &Value, out: &mut Vec<PartitionInfo>) {
    if node.get("type").and_then(|v| v.as_str()) == Some("part") {
        let name = node.get("name").and_then(|v| v.as_str()).unwrap_or("?");
        let fs_type = node.get("fstype").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let size_bytes = node.get("size").and_then(|v| v.as_u64()).unwrap_or(0);
        out.push(PartitionInfo {
            device: format!("/dev/{name}"),
            fs_type,
            size_gb: size_bytes / 1_000_000_000,
        });
    }
    if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
        for child in children {
            walk(child, out);
        }
    }
}

/// `lvs` (LVM2 en lecture) liste les volumes logiques, lecture libre s'il est
/// installé ; absent sur la plupart des postes de bureau sans LVM.
fn collect_lvm() -> Vec<LvmVolumeInfo> {
    let Some(text) = crate::command::run(
        "lvs",
        &["--noheadings", "--units", "g", "--nosuffix", "-o", "vg_name,lv_name,lv_size"],
    ) else {
        return Vec::new();
    };

    text.lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            let vg_name = fields.next()?.to_string();
            let lv_name = fields.next()?.to_string();
            let size_gb = fields.next()?.parse::<f64>().ok()? as u64;
            Some(LvmVolumeInfo { vg_name, lv_name, size_gb })
        })
        .collect()
}

/// `/proc/mdstat` (RAID logiciel `mdadm`) est en lecture libre.
/// Format : "md0 : active raid1 sdb1[1] sda1[0]".
fn collect_raid() -> Vec<RaidArrayInfo> {
    let Ok(text) = fs::read_to_string("/proc/mdstat") else {
        return Vec::new();
    };

    let mut arrays = Vec::new();
    for line in text.lines() {
        let Some((device, rest)) = line.split_once(" : ") else { continue };
        if !device.starts_with("md") {
            continue;
        }
        let mut parts = rest.split_whitespace();
        let state = parts.next().unwrap_or("?").to_string();
        let level = parts.next().unwrap_or("?").to_string();
        let devices = parts
            .map(|p| p.split('[').next().unwrap_or(p).to_string())
            .collect();
        arrays.push(RaidArrayInfo {
            device: format!("/dev/{device}"),
            level,
            state,
            devices,
        });
    }
    arrays
}

pub fn collect() -> StorageLayoutInfo {
    StorageLayoutInfo {
        partitions: collect_partitions(),
        lvm_volumes: collect_lvm(),
        raid_arrays: collect_raid(),
    }
}
