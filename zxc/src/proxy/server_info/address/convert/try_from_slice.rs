use crate::proxy::server_info::address::Address;
use crate::proxy::server_info::address::error::AddressError;

impl TryFrom<&[u8]> for Address {
    type Error = AddressError;

    fn try_from(val: &[u8]) -> Result<Self, Self::Error> {
        let addr_str = std::str::from_utf8(val)?;
        Self::try_from(addr_str)
    }
}
