use gethostname::gethostname;
use local_ip_address::list_afinet_netifas;
use std::net::IpAddr;
use sysinfo::{
    Disks, System,
};
use serde::Serialize;

// Memory information struct
#[derive(Serialize)]
struct MemoryInfo 
{
    total: u64,
    used: u64,
    swap_total: u64,
    swap_used: u64,
}

// Disks information struct 
#[derive(Serialize)]
struct DiskInfo
{
    name: String,
    kind: String,
    file_system: String,
    total_space: u64,
    available_space: u64,
}

#[derive(Serialize)]
struct NetworkInfo
{
    ipv4: Vec<String>,
    ipv6: Vec<String>,
}

// OS information
#[derive(Serialize)]
struct OSInfo 
{
    hostname: String,
    os: String,
    kernel: String,
}

// Agent information struct
#[derive(Serialize)]
struct AgentInfo 
{
    os: OSInfo,
    memory: MemoryInfo,
    network: NetworkInfo,
    disks: Vec<DiskInfo>,
}

fn converter(bytes: u64) -> (f64, &'static str)
{
    let mega: f64 = 1024.0 * 1024.0;
    let giga: f64 = 1024.0 * 1024.0 * 1024.0;
    let b = bytes as f64;

    let mut resultado;

    if b >= giga 
    {
        resultado = (b / giga, "GiB");
    } 
    else 
    {
        resultado = (b / mega, "MiB");
    }

    resultado

}

/* RAM and swap information:
    A função memory é usada para obter informações de memória e swap do sistema, retornando uma struct MemoryInfo com os valores totais e usados.
    A função show_memory é usada para exibir essas informações de forma legível, convertendo os valores de bytes para MiB ou GiB conforme necessário.
fn memory(sys: &System)
{
    let (val, unit) = converter(sys.total_memory());
    println!("total memory: {:.2} {}", val, unit);
    let (val, unit) = converter(sys.used_memory());
    println!("used memory : {:.2} {}", val, unit);
    let (val, unit) = converter(sys.total_swap());
    println!("total swap  : {:.2} {}", val, unit);
    let (val, unit) = converter(sys.used_swap());
    println!("used swap   : {:.2} {}", val, unit);
}
*/
fn memory(sys: &System) -> MemoryInfo 
{
    MemoryInfo 
    {
        total: sys.total_memory(),
        used: sys.used_memory(),
        swap_total: sys.total_swap(),
        swap_used: sys.used_swap(),
    }
}

// IP's and network information:
fn network() -> NetworkInfo
{
    let interfaces = list_afinet_netifas().unwrap();
    let mut ipv4_addresses = Vec::new();
    let mut ipv6_addresses = Vec::new();

    for (name, ip) in interfaces 
    {
        match ip 
        {
            IpAddr::V4(ipv4) => {
                if !ipv4.is_loopback() 
                {
                    ipv4_addresses.push(ipv4.to_string());
                }
            }

            IpAddr::V6(ipv6) => {
                if !ipv6.is_loopback() && !ipv6.is_unicast_link_local() 
                {
                    ipv6_addresses.push(ipv6.to_string());
                }
            }
        }
    }

    NetworkInfo 
    {
        ipv4: ipv4_addresses,
        ipv6: ipv6_addresses,
    }
}

// Disk information:
/*
fn disk(sys: &System)
{
    let disks = Disks::new_with_refreshed_list();

    for disk in disks.list() 
    {
        let(total, total_unit) = converter(disk.total_space());
        let(available, available_unit) = converter(disk.available_space());

       println!(
            "{} | {} | {} | Total: {:.2} {} | Livre: {:.2} {}",
            disk.name().to_string_lossy(),
            disk.kind(),
            disk.file_system().to_string_lossy(),
            total, total_unit,
            available, available_unit
        );
    }
}
*/
fn disks(disks: &Disks) -> Vec<DiskInfo>
{
    let mut infos = Vec::new();

    for disk in disks.list()
    {
        infos.push(DiskInfo {
            name: disk.name().to_string_lossy().into_owned(),
            kind: disk.kind().to_string(),
            file_system: disk.file_system().to_string_lossy().into_owned(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
        });
    }

    infos
}

fn os() -> OSInfo
{
    OSInfo 
    {
        hostname: gethostname().to_string_lossy().into_owned(),
        os: System::name().unwrap_or_else(|| "Unknown".to_string()),
        kernel: System::kernel_long_version(),
    }
}

// Show memory information
fn show_memory(info: &MemoryInfo)
{
    let (value, unit) = converter(info.total);
    println!("Total Memory: {:.2} {}", value, unit);

    let (value, unit) = converter(info.used);
    println!("Used Memory: {:.2} {}", value, unit);

    let (value, unit) = converter(info.swap_total);
    println!("Total Swap: {:.2} {}", value, unit);

    let (value, unit) = converter(info.swap_used);
    println!("Used Swap: {:.2} {}", value, unit);
}

fn show_disks(disks: &Vec<DiskInfo>)
{
    for disk in disks
    {
        let (total_value, total_unit) = converter(disk.total_space);
        let (available_value, available_unit) = converter(disk.available_space);

        println!(
            "{} | {} | {} | Total: {:.2} {} | Available: {:.2} {}",
            disk.name,
            disk.kind,
            disk.file_system,
            total_value, total_unit,
            available_value, available_unit
        );
    }
}

fn show_network(info: &NetworkInfo) 
{
    for ip in &info.ipv4 {
        println!("IPv4: {}", ip);
    }
    for ip in &info.ipv6 {
        println!("IPv6: {}", ip);
    }
}

/*
fn main() 
{
    let mut sys = System::new();
    sys.refresh_memory();

    println!("=> OS:");
    let os_info = os(&sys);
    let json = serde_json::to_string_pretty(&os_info).unwrap();
    println!("{}", json);

    println!("=> Memory:");
    let mem_info = memory(&sys);
    //show_memory(&mem_info);
    let json = serde_json::to_string_pretty(&mem_info).unwrap();
    println!("{}", json);

    println!("=> Network:");
    let network_info = network(&sys);
    //show_network(&network_info);
    let json = serde_json::to_string_pretty(&network_info).unwrap();
    println!("{}", json);

    println!("=> Disks:");
    let disks_list = Disks::new_with_refreshed_list();
    let disk_infos = disks(&disks_list);
    //show_disks(&disk_infos);
    let json = serde_json::to_string_pretty(&disk_infos).unwrap();
    println!("{}", json);
}
*/

fn main()
{
    let mut sys = System::new();
    sys.refresh_memory();

    let disks_list = Disks::new_with_refreshed_list();

    let agent = AgentInfo 
    {
        os: os(),
        memory: memory(&sys),
        network: network(),
        disks: disks(&disks_list),
    };

    let json = serde_json::to_string(&agent).unwrap();
    println!("{}", json);
}