mod try_from_slice;
mod try_from_str;
use super::AddressError;
mod try_from_str_scheme;

/* Description:
 *      Method to split host and port from an &str.
 *      input can be:
 *          - host:port
 *          - host without port, in which case return 0.
 *
 * Steps:
 *      1. rplit_once(':') to get host and port
 *      2. If host is empty, return error
 *      3. Parse port to u16, if no port is found, set port to 0
 *
 * Error:
 *      AddressError::NoHost    [2]
 *      AddressError::PortParse [3]
 */

pub fn split_host_and_port(addr: &str) -> Result<(&str, u16), AddressError> {
    let (host, port_str) = addr
        .rsplit_once(':')
        .unwrap_or((addr, ""));

    if host.is_empty() {
        return Err(AddressError::NoHost(addr.to_string()));
    }

    let port = if !port_str.is_empty() {
        port_str.parse::<u16>()?
    } else {
        0
    };

    Ok((host, port))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_split_with_port() {
        let (host, port) = split_host_and_port("www.google.com:8080").unwrap();
        assert_eq!(host, "www.google.com");
        assert_eq!(port, 8080);
    }

    #[test]
    fn test_split_without_port() {
        let (host, port) = split_host_and_port("www.google.com").unwrap();
        assert_eq!(host, "www.google.com");
        assert_eq!(port, 0);
    }

    #[test]
    fn test_split_with_empty_host() {
        let result = split_host_and_port(":8080");
        assert!(matches!(result, Err(AddressError::NoHost(_))));
    }

    #[test]
    fn test_split_large_port() {
        let result = split_host_and_port("www.google.com:65536");
        assert!(matches!(result, Err(AddressError::PortParse(_))));
    }
}
