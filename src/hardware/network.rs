use serde::Serialize;
use sysinfo::Networks;

#[derive(Serialize)]
pub struct NetworkInterfaceInfo {
    pub interface_name: String,
    pub received_bytes: u64,
    pub transmitted_bytes: u64,
}

pub fn collect() -> Vec<NetworkInterfaceInfo> {
    let networks = Networks::new_with_refreshed_list();
    networks
        .iter()
        .map(|(interface_name, data)| NetworkInterfaceInfo {
            interface_name: interface_name.clone(),
            received_bytes: data.received(),
            transmitted_bytes: data.transmitted(),
        })
        .collect()
}
