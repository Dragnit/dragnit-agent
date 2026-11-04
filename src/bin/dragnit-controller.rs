use ipnet::Ipv4Net;
use std::process::Command;
//use dragnit_agent::get_network_interfaces;
use dragnit_agent::DeviceInfo;

// Function to display network information
fn devices() -> Vec<DeviceInfo>
{
    let mut infos = Vec::new();

    let output = Command::new("ip")
        .args(["-4", "neigh", "show", "dev", "wlan0"])
        .output()
        .expect("Erro ao executar ip neigh");

    let texto = String::from_utf8_lossy(&output.stdout);

    // Aqui vamos percorrer as linhas
    for linha in texto.lines() 
    {
        let partes: Vec<&str> = linha.split_whitespace().collect();

        if partes.len() == 4 && partes[1] == "lladdr" 
        {
            let ip = partes[0].to_string();
            let mac = Some(partes[2].to_string());
            let hostname = None;
            let state = partes[3].to_string();

            infos.push(DeviceInfo
            {
                ip,
                mac,
                hostname,
                state,
            });
        }
    }

    infos
}

// Function to display neighbor information for a specific IP
fn neighbor(ip: &str) 
{
    let output = Command::new("ip")
        .args(["-4", "neigh", "show", ip, "dev", "wlan0"])
        .output()
        .expect("Erro ao consultar vizinho");

    let texto = String::from_utf8_lossy(&output.stdout);

    println!("{}", texto);
}

// Function to display network information
fn main() 
{
    let subnet: Ipv4Net = "192.168.1.0/24".parse().unwrap();
    let mut ips_ativos = Vec::new();

    for ip in subnet.hosts() 
    {
        let ip_texto = ip.to_string();

        // 1. Tentar contactar o dispositivo
        let resultado = Command::new("ping")
            .args(["-c", "1", "-W", "1", &ip_texto])
            .output()
            .expect("Erro ao executar ping");

        if resultado.status.success()
        {
            ips_ativos.push(ip);
            println!("IP: {} | Respondeu ao ping", ip);
        }

        // 2. Consultar imediatamente a tabela ARP
        neighbor(&ip_texto);
    }

    // 3. Mostrar os dispositivos ainda presentes na tabela
    let dispositivos = devices();

    for dispositivo in dispositivos 
    {
        println!(
            "IP: {} | MAC: {:?} | Estado: {}",
            dispositivo.ip,
            dispositivo.mac,
            dispositivo.state
        );
    }

    println!("IPs que responderam: {:?}", ips_ativos);
}