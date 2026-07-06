#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use serde::Serialize;

#[derive(Serialize)]
pub struct GpuInfo {
    pub name: String,
    pub vendor: Option<String>,
}

/// Approche native par OS plutôt que `wgpu` : évite une dépendance lourde
/// nécessitant des drivers Vulkan/Metal/DX12 même pour un simple inventaire,
/// ce qui serait instable en environnement headless/SSH.
pub fn collect() -> Vec<GpuInfo> {
    #[cfg(target_os = "linux")]
    {
        linux::collect()
    }
    #[cfg(target_os = "windows")]
    {
        windows::collect()
    }
    #[cfg(target_os = "macos")]
    {
        macos::collect()
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        Vec::new()
    }
}
