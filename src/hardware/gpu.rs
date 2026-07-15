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
    pub vram_mb: Option<u64>,
    pub driver_version: Option<String>,
}

/// Approche native par OS plutôt que `wgpu` : évite une dépendance lourde
/// nécessitant des drivers Vulkan/Metal/DX12 même pour un simple inventaire,
/// ce qui serait instable en environnement headless/SSH.
pub fn collect() -> Vec<GpuInfo> {
    crate::os_dispatch::dispatch_os!(linux::collect(), macos::collect(), windows::collect(), Vec::new())
}
