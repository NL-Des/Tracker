use serde::Serialize;
use sysinfo::Disks;

#[derive(Serialize)]
pub struct DiskInfo {
    pub name: String,
    pub kind: String,
    pub file_system: String,
    pub mount_point: String,
    pub is_removable: bool,
    pub total_gb: u64,
    pub used_gb: u64,
}

pub fn collect() -> Vec<DiskInfo> {
    let disks = Disks::new_with_refreshed_list();
    disks
        .iter()
        .map(|disk| {
            let total_gb = disk.total_space() / 1024 / 1024 / 1024;
            let available_gb = disk.available_space() / 1024 / 1024 / 1024;
            DiskInfo {
                name: disk.name().to_string_lossy().to_string(),
                kind: disk.kind().to_string(),
                file_system: disk.file_system().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                is_removable: disk.is_removable(),
                total_gb,
                used_gb: total_gb - available_gb,
            }
        })
        .collect()
}
