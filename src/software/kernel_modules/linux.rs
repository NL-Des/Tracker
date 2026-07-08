use super::KernelModuleInfo;
use std::fs;

/// `/proc/modules` est world-readable, équivalent à `lsmod` sans dépendre
/// d'un binaire externe. Format : "nom taille refcount deps state addr".
pub fn collect() -> Vec<KernelModuleInfo> {
    let Ok(contents) = fs::read_to_string("/proc/modules") else {
        return Vec::new();
    };

    contents
        .lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            let name = fields.next()?.to_string();
            let size_bytes = fields.next()?.parse().ok()?;
            Some(KernelModuleInfo { name, size_bytes })
        })
        .collect()
}
