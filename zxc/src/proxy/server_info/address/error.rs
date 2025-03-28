use std::num::ParseIntError;
use std::str::Utf8Error;

use rustls_pki_types::InvalidDnsNameError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AddressError {
    #[error("invalid str| {0}")]
    Utf8Error(#[from] Utf8Error),
    #[error("no host in address| {0}")]
    NoHost(String),
    #[error("unable to parse port| {0}")]
    PortParse(#[from] ParseIntError),
    #[error("invalid dnsname| {0}")]
    InvalidDnsName(#[from] InvalidDnsNameError),
}
