use super::GpuInfo;

fn parse_value(line: &str, key: &str) -> Option<String> {
    let line = line.trim();
    line.strip_prefix(key)?
        .strip_prefix(':')
        .map(|v| v.trim().to_string())
}

pub fn collect() -> Vec<GpuInfo> {
    let Some(text) = crate::command::run("system_profiler", &["SPDisplaysDataType"]) else {
        return Vec::new();
    };

    let mut gpus = Vec::new();
    let mut current_name: Option<String> = None;
    let mut current_vendor: Option<String> = None;
    let mut current_vram_mb: Option<u64> = None;
    let mut current_driver_version: Option<String> = None;

    for line in text.lines() {
        if let Some(name) = parse_value(line, "Chipset Model") {
            if let Some(prev_name) = current_name.take() {
                gpus.push(GpuInfo {
                    name: prev_name,
                    vendor: current_vendor.take(),
                    vram_mb: current_vram_mb.take(),
                    driver_version: current_driver_version.take(),
                });
            }
            current_name = Some(name);
        } else if let Some(vendor) = parse_value(line, "Vendor") {
            current_vendor = Some(vendor);
        } else if let Some(vram) = parse_value(line, "VRAM (Total)") {
            current_vram_mb = parse_vram_mb(&vram);
        } else if let Some(metal) = parse_value(line, "Metal Support") {
            // macOS n'expose pas de numéro de version driver classique ; le
            // support Metal (ex: "Metal 3") est l'information la plus proche
            // disponible sans privilège particulier.
            current_driver_version = Some(metal);
        }
    }
    if let Some(name) = current_name {
        gpus.push(GpuInfo {
            name,
            vendor: current_vendor,
            vram_mb: current_vram_mb,
            driver_version: current_driver_version,
        });
    }

    gpus
}

/// Convertit une valeur du type "1536 MB" ou "8 GB" en mégaoctets.
fn parse_vram_mb(value: &str) -> Option<u64> {
    let value = value.trim();
    if let Some(gb) = value.strip_suffix("GB").map(|s| s.trim()) {
        return gb.parse::<u64>().ok().map(|v| v * 1024);
    }
    if let Some(mb) = value.strip_suffix("MB").map(|s| s.trim()) {
        return mb.parse::<u64>().ok();
    }
    None
}
