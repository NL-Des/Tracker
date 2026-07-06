use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
pub struct OsInfo {
    pub name: Option<String>,
    pub kernel_version: Option<String>,
    pub os_version: Option<String>,
    pub host_name: Option<String>,
    pub uptime_seconds: u64,
}

pub fn collect() -> OsInfo {
    OsInfo {
        name: System::name(),
        kernel_version: System::kernel_version(),
        os_version: System::os_version(),
        host_name: System::host_name(),
        uptime_seconds: System::uptime(),
    }
}
