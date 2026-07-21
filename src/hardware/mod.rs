pub mod battery;
pub mod bluetooth_devices;
pub mod camera;
pub mod components;
pub mod cpu;
pub mod disks;
pub mod display_monitor;
pub mod fans;
pub mod gpu;
pub mod input_devices;
pub mod memory;
pub mod motherboard;
pub mod network;
pub mod optical_drives;
pub mod pci_devices;
pub mod peripherals;
pub mod power_profile;
pub mod printers;
pub mod storage_layout;
pub mod usb_devices;
pub mod wifi;

use serde::Serialize;
use sysinfo::System;

// ⚠️ Garder synchronisé avec `consent::HardwareConsent` (src/consent.rs) et ce
// tableau — voir `tests/consent_parity.rs`, qui échoue si l'un des trois diverge.
pub const HARDWARE_FIELDS: &[&str] = &[
    "cpu",
    "memory",
    "disks",
    "virtual_disks",
    "network",
    "wifi",
    "components",
    "batteries",
    "motherboard",
    "gpus",
    "pci_devices",
    "monitors",
    "optical_drives",
    "peripherals",
    "mice",
    "gamepads",
    "touchpads",
    "cameras",
    "usb_devices",
    "bluetooth_devices",
    "printers",
    "fans",
    "storage_layout",
    "power_profile",
];

#[derive(Serialize)]
pub struct HardwareInfo {
    pub cpu: cpu::CpuInfo,
    pub memory: memory::MemoryInfo,
    pub disks: Vec<disks::DiskInfo>,
    pub virtual_disks: Vec<disks::DiskInfo>,
    pub network: network::NetworkInfo,
    pub wifi: Vec<wifi::WifiNetworkInfo>,
    pub components: Vec<components::ComponentInfo>,
    pub batteries: Vec<battery::BatteryInfo>,
    pub motherboard: motherboard::MotherboardInfo,
    pub gpus: Vec<gpu::GpuInfo>,
    pub pci_devices: Vec<pci_devices::PciDeviceInfo>,
    pub monitors: Vec<display_monitor::MonitorInfo>,
    pub optical_drives: Vec<optical_drives::OpticalDriveInfo>,
    pub peripherals: Vec<peripherals::PeripheralInfo>,
    pub mice: Vec<input_devices::InputDeviceInfo>,
    pub gamepads: Vec<input_devices::InputDeviceInfo>,
    pub touchpads: Vec<input_devices::InputDeviceInfo>,
    pub cameras: Vec<camera::CameraInfo>,
    pub usb_devices: Vec<usb_devices::UsbDeviceInfo>,
    pub bluetooth_devices: Vec<bluetooth_devices::BluetoothDeviceInfo>,
    pub printers: Vec<printers::PrinterInfo>,
    pub fans: Vec<fans::FanInfo>,
    pub storage_layout: storage_layout::StorageLayoutInfo,
    pub power_profile: power_profile::PowerProfileInfo,
}

pub fn collect(sys: &System) -> HardwareInfo {
    let (disks, virtual_disks) = disks::collect();
    let input_devices = input_devices::collect();

    HardwareInfo {
        cpu: cpu::collect(sys),
        memory: memory::collect(sys),
        disks,
        virtual_disks,
        network: network::collect(),
        wifi: wifi::collect(),
        components: components::collect(),
        batteries: battery::collect(),
        motherboard: motherboard::collect(),
        gpus: gpu::collect(),
        pci_devices: pci_devices::collect(),
        monitors: display_monitor::collect(),
        optical_drives: optical_drives::collect(),
        peripherals: peripherals::collect(),
        mice: input_devices.mice,
        gamepads: input_devices.gamepads,
        touchpads: input_devices.touchpads,
        cameras: camera::collect(),
        usb_devices: usb_devices::collect(),
        bluetooth_devices: bluetooth_devices::collect(),
        printers: printers::collect(),
        fans: fans::collect(),
        storage_layout: storage_layout::collect(),
        power_profile: power_profile::collect(),
    }
}
