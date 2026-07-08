use super::KernelModuleInfo;
use std::process::Command;

/// `kextstat` liste les extensions noyau (kexts) chargées, sans root.
/// Format d'une ligne de données : "Index Refs Address Size Wired Name (Version) ...".
pub fn collect() -> Vec<KernelModuleInfo> {
    let Ok(output) = Command::new("kextstat").output() else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    let text = String::from_utf8_lossy(&output.stdout);

    text.lines()
        .skip(1)
        .filter_map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            let size_bytes = u64::from_str_radix(fields.get(3)?.trim_start_matches("0x"), 16).ok()?;
            let name = fields.get(5)?.to_string();
            Some(KernelModuleInfo { name, size_bytes })
        })
        .collect()
}
