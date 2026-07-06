pub mod env_vars;
pub mod installed_apps;
pub mod os_info;
pub mod processes;
pub mod users;

use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
pub struct SoftwareInfo {
    pub os: os_info::OsInfo,
    pub processes: processes::ProcessSummary,
    pub users: Vec<users::UserAccountInfo>,
    pub env_vars: Vec<env_vars::EnvVarInfo>,
    pub installed_apps: Vec<installed_apps::InstalledAppInfo>,
}

pub fn collect(sys: &System) -> SoftwareInfo {
    SoftwareInfo {
        os: os_info::collect(),
        processes: processes::collect(sys),
        users: users::collect(),
        env_vars: env_vars::collect(),
        installed_apps: installed_apps::collect(),
    }
}
