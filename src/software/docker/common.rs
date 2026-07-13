use super::{DockerImageInfo, DockerVolumeInfo};
use serde_json::Value;

/// Le CLI `docker` a la même surface sur Linux/macOS/Windows (Docker Desktop),
/// donc cette logique est partagée par les 3 implémentations OS.
pub fn collect_images() -> Vec<DockerImageInfo> {
    let Some(text) = crate::command::run("docker", &["image", "ls", "--format", "{{json .}}"]) else {
        return Vec::new();
    };

    text.lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .map(|v| DockerImageInfo {
            repository: field(&v, "Repository"),
            tag: field(&v, "Tag"),
            image_id: field(&v, "ID"),
            size: field(&v, "Size"),
            created: field(&v, "CreatedSince"),
        })
        .collect()
}

pub fn collect_volumes() -> Vec<DockerVolumeInfo> {
    let Some(text) = crate::command::run("docker", &["volume", "ls", "--format", "{{json .}}"]) else {
        return Vec::new();
    };

    text.lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .map(|v| {
            let name = field(&v, "Name");
            let driver = field(&v, "Driver");
            let mountpoint = inspect_mountpoint(&name);
            DockerVolumeInfo { name, driver, mountpoint }
        })
        .collect()
}

fn field(value: &Value, key: &str) -> String {
    value.get(key).and_then(|v| v.as_str()).unwrap_or("?").to_string()
}

fn inspect_mountpoint(name: &str) -> Option<String> {
    let text = crate::command::run(
        "docker",
        &["volume", "inspect", "--format", "{{json .Mountpoint}}", name],
    )?;
    serde_json::from_str::<String>(text.trim()).ok()
}
