use crate::proxy::server_info::address::Address;
use crate::proxy::server_info::address::error::AddressError;
use crate::proxy::server_info::scheme::Scheme;

impl TryFrom<(&str, Scheme)> for Address {
    type Error = AddressError;

    fn try_from((host, scheme): (&str, Scheme)) -> Result<Self, Self::Error> {
        let mut address = Address::try_from(host)?;
        // If port is 0, set it to scheme default
        if address.port() == 0 {
            address.set_port(scheme.default_port());
        }
        Ok(address)
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_address_from_str_scheme_http() {
        let address = Address::try_from(("127.0.0.1", Scheme::Http)).unwrap();
        assert_eq!(
            address,
            Address::Socket(SocketAddr::from_str("127.0.0.1:80").unwrap())
        )
    }

    #[test]
    fn test_address_from_str_scheme_https() {
        let address = Address::try_from(("127.0.0.1", Scheme::Https)).unwrap();
        assert_eq!(address, Address::Socket("127.0.0.1:443".parse().unwrap()))
    }

    #[test]
    fn test_address_from_str_scheme_http_no_port() {
        let address = Address::try_from(("127.0.0.1", Scheme::Http)).unwrap();
        assert_eq!(address, Address::Socket("127.0.0.1:80".parse().unwrap()))
    }

    #[test]
    fn test_address_from_str_scheme_https_no_port() {
        let address = Address::try_from(("127.0.0.1", Scheme::Https)).unwrap();
        assert_eq!(address, Address::Socket("127.0.0.1:443".parse().unwrap()))
    }

    #[test]
    fn test_address_from_str_scheme_http_with_port() {
        let address =
            Address::try_from(("127.0.0.1:8080", Scheme::Http)).unwrap();
        assert_eq!(address, Address::Socket("127.0.0.1:8080".parse().unwrap()))
    }

    #[test]
    fn test_address_from_str_scheme_https_with_port() {
        let address =
            Address::try_from(("127.0.0.1:8443", Scheme::Https)).unwrap();
        assert_eq!(address, Address::Socket("127.0.0.1:8443".parse().unwrap()))
    }
}
