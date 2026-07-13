use super::PciDeviceInfo;

/// `lspci` (sans `-k`/`-v`) est en lecture seule et ne nécessite pas root.
/// Format d'une ligne : "0000:00:02.0 VGA compatible controller: Intel Corporation ...".
pub fn collect() -> Vec<PciDeviceInfo> {
    let Some(text) = crate::command::run("lspci", &[]) else {
        return Vec::new();
    };

    text.lines()
        .filter_map(|line| {
            let (_, rest) = line.split_once(' ')?;
            let (class, name) = rest.split_once(':')?;
            Some(PciDeviceInfo {
                name: name.trim().to_string(),
                class: class.trim().to_string(),
            })
        })
        .collect()
}
