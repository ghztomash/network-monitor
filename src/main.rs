use dns_lookup::{getaddrinfo, lookup_addr, lookup_host, AddrInfoHints};
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use sysinfo::{Pid, System};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let show_host_name = true; // true: show numeric addresses, false: show resolved addresses
    let show_service_name = true; // true: show service name, false: show port number
    let show_process_name = true; // true: show process name, false: show process ID

    let af_flags = AddressFamilyFlags::IPV4;
    // let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP;
    // let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets_info = get_sockets_info(af_flags, proto_flags)?;

    let mut system = System::new();
    system.refresh_all();

    for si in sockets_info {
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => {
                let local_addr = if !show_host_name {
                    format!("{}", tcp_si.local_addr)
                } else {
                    let ip = tcp_si.local_addr;
                    let host = lookup_addr(&ip).unwrap_or(ip.to_string());
                    format!("{}", host)
                };
                let remoe_addr = if !show_host_name {
                    format!("{}", tcp_si.remote_addr)
                } else {
                    let ip = tcp_si.remote_addr;
                    let host = lookup_addr(&ip).unwrap_or(ip.to_string());
                    format!("{}", host)
                };
                let local_port = if !show_service_name {
                    format!("awfwa{}", tcp_si.local_port)
                } else {
                    let hints = AddrInfoHints {
                        socktype: 1,
                        ..AddrInfoHints::default()
                    };
                    let port = tcp_si.local_port.to_string();
                    // need to use getservbyport() to get the port number
                    let serv = getaddrinfo(None, Some(&port), Some(hints))
                        .unwrap()
                        .next()
                        .unwrap()
                        .unwrap()
                        .canonname
                        .unwrap_or(port);
                    format!("{}", serv)
                };
                let process_name = if show_process_name {
                    let pids = si.associated_pids;
                    let process = pids
                        .iter()
                        .map(|pid| {
                            system
                                .process(Pid::from(*pid as usize))
                                .unwrap()
                                .name()
                                .to_string()
                        })
                        .collect::<Vec<String>>();
                    process
                } else {
                    si.associated_pids
                        .iter()
                        .map(|pid| pid.to_string())
                        .collect::<Vec<String>>()
                };
                println!(
                    "TCP {}:{} -> {}:{} {:?} - {}",
                    local_addr,
                    local_port,
                    remoe_addr,
                    tcp_si.remote_port,
                    process_name,
                    tcp_si.state
                );
            }
            ProtocolSocketInfo::Udp(udp_si) => println!(
                "UDP {}:{} -> *:* {:?}",
                udp_si.local_addr, udp_si.local_port, si.associated_pids
            ),
        }
    }

    Ok(())
}
