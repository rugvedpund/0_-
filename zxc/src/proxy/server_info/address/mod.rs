use std::fmt::{Display, Formatter};
use std::net::SocketAddr;
pub mod error;
use error::AddressError;
use oneone::Request;
use oneone::abnf::FORWARD_SLASH;
use rustls_pki_types::{InvalidDnsNameError, ServerName};

use super::scheme::Scheme;
mod convert;

// Enum to represent the different address types of the server.
// The DNS variant holds host and port as a tuple.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Address {
    Socket(SocketAddr),
    Dns((String, u16)),
}

impl Address {
    pub fn port(&self) -> u16 {
        match self {
            Address::Socket(socket_addr) => socket_addr.port(),
            Address::Dns((_, port)) => *port,
        }
    }

    pub fn set_port(&mut self, port: u16) {
        match self {
            Address::Socket(socket_addr) => {
                *socket_addr = SocketAddr::new(socket_addr.ip(), port)
            }
            Address::Dns((_, old_port)) => *old_port = port,
        }
    }

    /* Description:
     *      Method to get ServerName from Address.
     *      Used to perform tls handshake.
     *
     * Steps:
     *      1. If sni is some, try to convert str to ServerName
     *      2. else match Address
     *          a. If SocketAddr, try to convert SocketAddr to ServerName by
     *              calling ServerName::From()
     *          b. If Dns, try to convert Dns to ServerName by calling
     *              ServerName::try_from()
     */

    pub fn parse_sni<'a>(
        &'a self,
        sni: Option<&'a str>,
    ) -> Result<ServerName<'a>, InvalidDnsNameError> {
        if let Some(sni) = sni {
            return ServerName::try_from(sni);
        }
        match self {
            Address::Socket(socket_addr) => {
                Ok(ServerName::from(socket_addr.ip()))
            }
            Address::Dns((host, _)) => ServerName::try_from(host.as_str()),
        }
    }

    /* Description:
     *      Method to check if host and sni are equal
     *
     *  https://docs.rs/rustls-pki-types/latest/rustls_pki_types/enum.IpAddr.html
     *
     *  ServerName uses no-std IpAddr, so we need to convert it to String
     */
    pub fn is_host_sni_equal(&self, sni: &ServerName) -> bool {
        match self {
            Address::Socket(addr) => addr.ip().to_string() == sni.to_str(),
            Address::Dns((dns, _)) => *dns == sni.to_str(),
        }
    }

    /* Description:
     *      Method to convert Address to String
     *
     * Steps:
     *      1. If port is scheme's default port, return only ip/host without
     *         port
     *      2. Else return ip/host:port
     */
    pub fn to_string_from_scheme(&self, scheme: Scheme) -> String {
        if self.port() == scheme.default_port() {
            return match self {
                Address::Socket(socket_addr) => socket_addr.ip().to_string(),
                Address::Dns((host, _)) => host.to_string(),
            };
        }
        self.to_string()
    }
}

// Display implementation for Address
impl Display for Address {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Address::Socket(socket_addr) => write!(f, "{}", socket_addr),
            Address::Dns((host, port)) => write!(f, "{}:{}", host, port),
        }
    }
}

/* Description:
 *      Get Address from Request Info Line for both http and https.
 *
 * Steps:
 *      1. If tls, its a CONNECT request in which case the uri is the address.
 *      2. If not, its in format http://host:port/
 *          a. find index '/' in uri "http://" and trailing '/'
 *          b. Separe http://host:port from trailing '/uri'
 *          c. Call Address::try_from(&[u8])
 *          d. If no port found, set port to 80.
 *
 *      3. If host.len() > request.method.len(), copy request to the host
 *      overwriting the host and set the request.method to host. This avoids
 *      a unnecessary copy of the request.
 */

pub fn get_address(
    request: &mut Request,
    tls: bool,
) -> Result<Address, AddressError> {
    let address = if tls {
        Address::try_from(request.uri_as_mut().split().as_ref())?
    } else {
        // a. find index '/'
        let fs_index_vec = request
            .uri_as_mut()
            .iter()
            .enumerate()
            .filter(|&(_, &r)| r == FORWARD_SLASH[0])
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();
        // b. Separate http://host:port from trailing '/uri'
        let mut host = request
            .uri_as_mut()
            .split_to(fs_index_vec[2]);
        // c. Remove "http://"
        let mut address = Address::try_from(&host[fs_index_vec[1] + 1..])?;
        // e. If no port found, set port to 80.
        if address.port() == 0 {
            address.set_port(80);
        }

        let mlen = request.method_raw().len();
        // 3
        if host.len() > mlen {
            let start_index = host.len() - mlen;
            let _ = host.split_to(start_index);
            host.clear();
            host.extend_from_slice(request.method_raw());
            request.set_method_raw(host);
        }
        address
    };
    Ok(address)
}

#[cfg(test)]
mod tests {
    use std::ops::Range;
    use std::str::FromStr;

    use bytes::BytesMut;
    use oneone::InfoLine;

    use super::*;

    #[test]
    fn test_address_set_port() {
        let mut address = Address::try_from("127.0.0.1:9001").unwrap();
        address.set_port(8080);
        assert_eq!(
            address,
            Address::Socket(SocketAddr::from_str("127.0.0.1:8080").unwrap())
        )
    }

    #[test]
    fn test_address_from_request_http_ip_get() {
        let info_line = "GET http://127.0.0.1:8080/echo HTTP/1.1\r\n";
        let buf = BytesMut::from(info_line);
        let initial_ptr_range = buf.as_ptr_range();
        let mut request = Request::build_infoline(buf).unwrap();
        let address = get_address(&mut request, false).unwrap();
        assert_eq!(
            address,
            Address::Socket(SocketAddr::from_str("127.0.0.1:8080").unwrap())
        );
        let result = request.into_data();
        let verify = "GET /echo HTTP/1.1\r\n";
        assert_eq!(result, BytesMut::from(verify));
        let final_ptr = result.as_ptr_range();
        let url = "http://127.0.0.1:8080";
        let expected_start_ptr =
            unsafe { initial_ptr_range.start.add(url.len()) };

        let expected_ptr_range = Range {
            start: expected_start_ptr,
            end: initial_ptr_range.end,
        };

        assert_eq!(final_ptr, expected_ptr_range);
    }

    #[test]
    fn test_address_from_request_http_ip_post() {
        let info_line = "POST http://127.0.0.1:8080/echo HTTP/1.1\r\n";
        let buf = BytesMut::from(info_line);
        let initial_ptr_range = buf.as_ptr_range();
        let mut request = Request::build_infoline(buf).unwrap();
        let address = get_address(&mut request, false).unwrap();
        assert_eq!(
            address,
            Address::Socket(SocketAddr::from_str("127.0.0.1:8080").unwrap())
        );
        let result = request.into_data();
        let verify = "POST /echo HTTP/1.1\r\n";
        assert_eq!(result, BytesMut::from(verify));
        let final_ptr = result.as_ptr_range();
        let url = "http://127.0.0.1:8080";
        let expected_start_ptr =
            unsafe { initial_ptr_range.start.add(url.len()) };
        let expected_ptr_range = Range {
            start: expected_start_ptr,
            end: initial_ptr_range.end,
        };
        assert_eq!(final_ptr, expected_ptr_range);
    }

    #[test]
    fn test_address_from_request_https_ip() {
        let info_line = "CONNECT 127.0.0.1:8080 HTTP/1.1\r\n";
        let buf = BytesMut::from(info_line);
        let mut request = Request::build_infoline(buf).unwrap();
        let address = get_address(&mut request, true).unwrap();
        assert_eq!(
            address,
            Address::Socket(SocketAddr::from_str("127.0.0.1:8080").unwrap())
        )
    }

    #[test]
    fn test_address_from_request_http_dns() {
        let info_line = "GET http://www.google.com/echo HTTP/1.1\r\n";
        let buf = BytesMut::from(info_line);
        let mut request = Request::build_infoline(buf).unwrap();
        let address = get_address(&mut request, false).unwrap();
        assert_eq!(address, Address::Dns(("www.google.com".to_string(), 80)))
    }

    #[test]
    fn test_address_from_request_https_dns() {
        let info_line = "CONNECT www.google.com:443 HTTP/1.1\r\n";
        let buf = BytesMut::from(info_line);
        let mut request = Request::build_infoline(buf).unwrap();
        let address = get_address(&mut request, true).unwrap();
        assert_eq!(address, Address::Dns(("www.google.com".to_string(), 443)))
    }

    #[test]
    fn test_host_sni_equal_ip() {
        let server_name = ServerName::try_from("127.0.0.1").unwrap();
        let address = Address::Socket("127.0.0.1:80".parse().unwrap());
        assert!(address.is_host_sni_equal(&server_name));
    }

    #[test]
    fn test_host_sni_equal_dns() {
        let server_name = ServerName::try_from("www.google.com").unwrap();
        let address = Address::Dns(("www.google.com".to_string(), 80));
        assert!(address.is_host_sni_equal(&server_name));
    }

    #[test]
    fn test_host_sni_equal_dns_to_ip() {
        let server_name = ServerName::try_from("127.0.0.1").unwrap();
        let address = Address::Dns(("www.google.com".to_string(), 80));
        assert!(!address.is_host_sni_equal(&server_name));
    }

    #[test]
    fn test_to_string_from_scheme_default_http_ip() {
        let address = Address::Socket("127.0.0.1:80".parse().unwrap());
        assert_eq!(address.to_string_from_scheme(Scheme::Http), "127.0.0.1")
    }

    #[test]
    fn test_to_string_from_scheme_default_https_ip() {
        let address = Address::Socket("127.0.0.1:443".parse().unwrap());
        assert_eq!(address.to_string_from_scheme(Scheme::Https), "127.0.0.1")
    }

    #[test]
    fn test_to_string_from_scheme_http_dns() {
        let address = Address::Dns(("www.google.com".to_string(), 80));
        assert_eq!(
            address.to_string_from_scheme(Scheme::Http),
            "www.google.com"
        )
    }

    #[test]
    fn test_to_string_from_scheme_https_dns() {
        let address = Address::Dns(("www.google.com".to_string(), 443));
        assert_eq!(
            address.to_string_from_scheme(Scheme::Https),
            "www.google.com"
        )
    }

    #[test]
    fn test_to_string_from_scheme_http_ip_port() {
        let address = Address::Socket("127.0.0.1:8080".parse().unwrap());
        assert_eq!(
            address.to_string_from_scheme(Scheme::Http),
            "127.0.0.1:8080"
        )
    }

    #[test]
    fn test_to_string_from_scheme_https_ip_port() {
        let address = Address::Socket("127.0.0.1:8080".parse().unwrap());
        assert_eq!(
            address.to_string_from_scheme(Scheme::Https),
            "127.0.0.1:8080"
        )
    }

    #[test]
    fn test_to_string_from_scheme_http_dns_port() {
        let address = Address::Dns(("www.google.com".to_string(), 8080));
        assert_eq!(
            address.to_string_from_scheme(Scheme::Http),
            "www.google.com:8080"
        )
    }

    #[test]
    fn test_to_string_from_scheme_https_dns_port() {
        let address = Address::Dns(("www.google.com".to_string(), 8080));
        assert_eq!(
            address.to_string_from_scheme(Scheme::Https),
            "www.google.com:8080"
        )
    }
}
