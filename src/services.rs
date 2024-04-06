use std::io;
use std::net::IpAddr;
use std::net::SocketAddr;

use dns_lookup::{getaddrinfo, getnameinfo, AddrInfoHints};

/// look up the port number for a given service name.
pub fn lookup_port(service: &str, hints: Option<AddrInfoHints>) -> io::Result<Vec<u16>> {
    println!("service: {:?}", service);
    match getaddrinfo(Some("127.0.0.1"), Some(service), hints) {
        Ok(addrs) => {
            let addrs: io::Result<Vec<_>> = addrs
                .map(|r| {
                    r.map(|a| {
                        println!("{:?}", a);
                        a.sockaddr.port()
                    })
                })
                .collect();
            addrs
        }
        Err(e) => Err(e)?,
    }
}

/// Lookup the service and host name for a given connection.
pub fn lookup(addr: &IpAddr, port: u16) -> io::Result<(String, String)> {
    let socket: SocketAddr = (*addr, port).into();
    match getnameinfo(&socket, 0) {
        Ok((hostname, service)) => Ok((hostname, service)),
        Err(e) => Err(e)?,
    }
}

/// Lookup the service name for a given port number.
pub fn lookup_service(port: u16) -> io::Result<String> {
    let ip: IpAddr = "127.0.0.1".parse().unwrap();
    let socket: SocketAddr = (ip, port).into();
    match getnameinfo(&socket, 0) {
        Ok((_, service)) => Ok(service),
        Err(e) => Err(e)?,
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getservbyname() {
        let hints = Some(AddrInfoHints {
            socktype: 1,
            protocol: 6,
            ..AddrInfoHints::default()
        });
        assert_eq!(lookup_port("http", hints).unwrap(), vec![80]);
        assert_eq!(lookup_port("https", hints).unwrap(), vec![443]);
        assert_eq!(lookup_port("ssh", hints).unwrap(), vec![22]);
        assert!(lookup_port("abc", hints).is_err());
    }

    #[test]
    fn test_getservbyport() {
        assert_eq!(lookup_service(80).unwrap(), "http".to_string());
        assert_eq!(lookup_service(443).unwrap(), "https".to_string());
        assert_eq!(lookup_service(22).unwrap(), "ssh".to_string());
        assert_eq!(lookup_service(0).unwrap(), "0".to_string());
    }
}
