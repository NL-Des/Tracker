use super::common::collect_virtualbox;
use super::VirtualMachineInfo;

pub fn collect() -> Vec<VirtualMachineInfo> {
    collect_virtualbox()
}
