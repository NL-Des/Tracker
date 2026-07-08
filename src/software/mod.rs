pub mod autostart;
pub mod desktop_env;
pub mod dev_runtimes;
pub mod env_vars;
pub mod installed_apps;
pub mod kernel_modules;
pub mod network_connections;
pub mod os_info;
pub mod packages;
pub mod processes;
pub mod scheduled_tasks;
pub mod services;
pub mod update_history;
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
    pub dev_runtimes: Vec<dev_runtimes::DevRuntimeInfo>,
    pub services: Vec<services::ServiceInfo>,
    pub scheduled_tasks: Vec<scheduled_tasks::ScheduledTaskInfo>,
    pub autostart_entries: Vec<autostart::AutostartEntryInfo>,
    pub package_managers: Vec<packages::PackageManagerInfo>,
    pub network_connections: Vec<network_connections::NetworkConnectionInfo>,
    pub desktop_environment: desktop_env::DesktopEnvironmentInfo,
    pub update_history: Vec<update_history::UpdateHistoryEntryInfo>,
    pub kernel_modules: Vec<kernel_modules::KernelModuleInfo>,
}

pub fn collect(sys: &System) -> SoftwareInfo {
    SoftwareInfo {
        os: os_info::collect(),
        processes: processes::collect(sys),
        users: users::collect(),
        env_vars: env_vars::collect(),
        installed_apps: installed_apps::collect(),
        dev_runtimes: dev_runtimes::collect(),
        services: services::collect(),
        scheduled_tasks: scheduled_tasks::collect(),
        autostart_entries: autostart::collect(),
        package_managers: packages::collect(),
        network_connections: network_connections::collect(),
        desktop_environment: desktop_env::collect(),
        update_history: update_history::collect(),
        kernel_modules: kernel_modules::collect(),
    }
}
