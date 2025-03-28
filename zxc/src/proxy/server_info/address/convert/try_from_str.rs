use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use super::split_host_and_port;
use crate::proxy::server_info::address::Address;
use crate::proxy::server_info::address::error::AddressError;

impl TryFrom<&str> for Address {
    type Error = AddressError;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        // 1. Try to convert &str to SocketAddr
        if let Ok(socket_addr) = SocketAddr::from_str(val) {
            return Ok(Address::Socket(socket_addr));
        }

        // 2. Check if ipaddress
        if let Ok(ip) = IpAddr::from_str(val) {
            return Ok(Address::Socket(SocketAddr::new(ip, 0)));
        }

        // Else, treat it as DNS (host, port)
        let (host, port) = split_host_and_port(val)?;
        Ok(Address::Dns((host.to_string(), port)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_from_empty_str() {
        let address = Address::try_from("");
        assert!(matches!(address, Err(AddressError::NoHost(_))));
    }

    #[test]
    fn test_address_from_str_ip_only() {
        let address = Address::try_from("127.0.0.1").unwrap();
        assert_eq!(
            address,
            Address::Socket(SocketAddr::from_str("127.0.0.1:0").unwrap())
        )
    }

    #[test]
    fn test_address_from_str_ip_with_port() {
        let address = Address::try_from("127.0.0.1:8080").unwrap();
        assert_eq!(
            address,
            Address::Socket(SocketAddr::from_str("127.0.0.1:8080").unwrap())
        )
    }

    #[test]
    fn test_address_from_str_dns_only() {
        let address = Address::try_from("www.google.com").unwrap();
        assert_eq!(address, Address::Dns(("www.google.com".to_string(), 0)))
    }

    #[test]
    fn test_address_from_str_dns_with_port() {
        let address = Address::try_from("www.google.com:8080").unwrap();
        assert_eq!(address, Address::Dns(("www.google.com".to_string(), 8080)))
    }
}
