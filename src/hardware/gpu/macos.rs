use super::GpuInfo;
use std::process::Command;

fn parse_value(line: &str, key: &str) -> Option<String> {
    let line = line.trim();
    line.strip_prefix(key)?
        .strip_prefix(':')
        .map(|v| v.trim().to_string())
}

pub fn collect() -> Vec<GpuInfo> {
    let Ok(output) = Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .output()
    else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(&output.stdout);

    let mut gpus = Vec::new();
    let mut current_name: Option<String> = None;
    let mut current_vendor: Option<String> = None;

    for line in text.lines() {
        if let Some(name) = parse_value(line, "Chipset Model") {
            if let Some(prev_name) = current_name.take() {
                gpus.push(GpuInfo {
                    name: prev_name,
                    vendor: current_vendor.take(),
                });
            }
            current_name = Some(name);
        } else if let Some(vendor) = parse_value(line, "Vendor") {
            current_vendor = Some(vendor);
        }
    }
    if let Some(name) = current_name {
        gpus.push(GpuInfo {
            name,
            vendor: current_vendor,
        });
    }

    gpus
}
