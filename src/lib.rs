use serde::Serialize;
use local_ip_address::list_afinet_netifas;

// Memory information struct
#[derive(Serialize)]
pub struct MemoryInfo 
{
    pub total: u64,
    pub used: u64,
    pub swap_total: u64,
    pub swap_used: u64,
}

// Disks information struct 
#[derive(Serialize)]
pub struct DiskInfo
{
    pub name: String,
    pub kind: String,
    pub file_system: String,
    pub total_space: u64,
    pub available_space: u64,
}

#[derive(Serialize)]
pub struct NetworkInfo
{
    pub ipv4: Vec<String>,
    pub ipv6: Vec<String>,
}

// OS information
#[derive(Serialize)]
pub struct OSInfo 
{
    pub hostname: String,
    pub os: String,
    pub kernel: String,
}

// Agent information struct
#[derive(Serialize)]
pub struct AgentInfo 
{
    pub os: OSInfo,
    pub memory: MemoryInfo,
    pub network: NetworkInfo,
    pub disks: Vec<DiskInfo>,
}

// Device information struct
#[derive(Serialize)]
pub struct DeviceInfo
{
    pub ip: String,
    pub mac: Option<String>,
    pub hostname: Option<String>,
    pub state: String,
}

// get network information
pub fn get_network_interfaces() -> Vec<String> 
{
    let interfaces = list_afinet_netifas().unwrap();

    let mut addresses = Vec::new();

    for (name, ip) in interfaces {
        addresses.push(format!("{}: {}", name, ip));
    }

    addresses
}