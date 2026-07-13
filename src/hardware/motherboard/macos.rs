use super::MotherboardInfo;

fn parse_field(output: &str, key: &str) -> Option<String> {
    output.lines().find_map(|line| {
        let line = line.trim();
        line.strip_prefix(key)?
            .strip_prefix(':')
            .map(|v| v.trim().to_string())
    })
}

pub fn collect() -> MotherboardInfo {
    let Some(text) = crate::command::run("system_profiler", &["SPHardwareDataType"]) else {
        return MotherboardInfo::default();
    };

    MotherboardInfo {
        vendor: Some("Apple".to_string()),
        model: parse_field(&text, "Model Identifier")
            .or_else(|| parse_field(&text, "Model Name")),
        version: None,
        bios_vendor: Some("Apple".to_string()),
        bios_version: parse_field(&text, "Boot ROM Version"),
        bios_date: None,
        machine_uuid: parse_field(&text, "Hardware UUID"),
        secure_boot: None,
    }
}
