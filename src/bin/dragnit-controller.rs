
use ipnet::Ipv4Net;
use std::process::Command;
use std::thread;
use std::net::Ipv4Addr;
use std::time::Instant;
use if_addrs::{get_if_addrs, IfAddr};
use dragnit_agent::{DeviceInfo, NetworkConfig};

// Descobre a interface e a sub-rede do ASUS
fn detect_network() -> Option<NetworkConfig> 
{
    let interfaces = get_if_addrs().ok()?;

    for iface in interfaces 
    {
        if iface.name != "wlan0" 
        {
            continue;
        }

        if let IfAddr::V4(v4) = iface.addr 
        {
            let prefix = u32::from(v4.netmask).count_ones() as u8;
            let subnet = Ipv4Net::new(v4.ip, prefix).ok()?.trunc();

            return Some(NetworkConfig 
            {
                interface: iface.name,
                subnet,
            });
        }
    }

    None
}

// Consulta o MAC e o estado de um IP
fn neighbor(ip: &str, interface: &str) -> Option<DeviceInfo> 
{
    let output = Command::new("ip")
        .args(["-4", "neigh", "show", ip, "dev", interface])
        .output()
        .ok()?;

    if !output.status.success() 
    {
        return None;
    }

    let texto = String::from_utf8_lossy(&output.stdout);
    let partes: Vec<&str> = texto.split_whitespace().collect();

    // Versão simples para o formato que observámos no ASUS
    if partes.len() >= 4 && partes[1] == "lladdr" 
    {
        return Some(DeviceInfo 
        {
            ip: partes[0].to_string(),
            mac: Some(partes[2].to_string()),
            hostname: None,
            state: partes[partes.len() - 1].to_string(),
        });
    }

    None
}

// Cada thread usa esta função para pesquisar os seus IPs
fn scan_ips(ips: Vec<Ipv4Addr>, interface: String,) -> (Vec<Ipv4Addr>, Vec<DeviceInfo>) 
{
    let mut ips_ativos = Vec::new();
    let mut dispositivos = Vec::new();

    for ip in ips 
    {
        let ip_texto = ip.to_string();

        let resultado = Command::new("ping").args(["-c", "1", "-W", "1", &ip_texto]).output();

        if let Ok(resultado) = resultado 
        {
            if resultado.status.success() 
            {
                println!("IP: {} | Respondeu ao ping", ip);
                ips_ativos.push(ip);
            }
        }

        // Guarda o MAC imediatamente, antes de a entrada expirar
        if let Some(dispositivo) = neighbor(&ip_texto, &interface) 
        {
            dispositivos.push(dispositivo);
        }
    }

    (ips_ativos, dispositivos)
}

// Divide a pesquisa em duas threads e junta os resultados
fn scan_network(network: &NetworkConfig,) -> (Vec<Ipv4Addr>, Vec<DeviceInfo>) 
{
    let ips: Vec<Ipv4Addr> = network.subnet.hosts().collect();
    let meio = ips.len() / 2;

    let primeira_metade = ips[..meio].to_vec();
    let segunda_metade = ips[meio..].to_vec();

    let interface1 = network.interface.clone();
    let interface2 = network.interface.clone();

    let handle1 = thread::spawn(move || {
        scan_ips(primeira_metade, interface1)
    });

    let handle2 = thread::spawn(move || {
        scan_ips(segunda_metade, interface2)
    });

    // Espera que as duas threads terminem
    let (mut ativos1, mut dispositivos1) = handle1.join().unwrap();
    let (ativos2, dispositivos2) = handle2.join().unwrap();

    // Junta os resultados
    ativos1.extend(ativos2);
    dispositivos1.extend(dispositivos2);

    (ativos1, dispositivos1)
}

fn arp_scan() {
    let output = Command::new("arp-scan").args(["--interface=wlan0", "--retry=3", "--localnet"]).output().expect("Não foi possível executar arp-scan");

    if !output.status.success() 
    {
        eprintln!("Erro ao executar o comando arp-scan");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        return;
    }

    let texto = String::from_utf8_lossy(&output.stdout);

    println!("Resultado do scan ARP:\n{}", texto);
}

fn main() 
{
    arp_scan();
}

/*
fn main() 
{
    let network = detect_network().expect("ERRO: Não foi possível detetar a rede!");

    println!("Interface: {}", network.interface);
    println!("Sub-rede: {}", network.subnet);
    println!("A iniciar scan com 2 threads...\n");

    let inicio = Instant::now();

    let (ips_ativos, dispositivos) = scan_network(&network);

    println!("\n===== RESULTADOS =====");

    for dispositivo in &dispositivos {
        println!(
            "IP: {} | MAC: {:?} | Estado: {}",
            dispositivo.ip,
            dispositivo.mac,
            dispositivo.state
        );
    }

    println!("\nIPs que responderam: {:?}", ips_ativos);
    println!("Dispositivos com MAC: {}", dispositivos.len());
    println!("Tempo do scan: {:.2?}", inicio.elapsed());
}
*/