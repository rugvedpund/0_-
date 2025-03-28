use std::path::PathBuf;

use crate::proxy::server_info::ServerInfo;
use crate::proxy::server_info::address::error::AddressError;
use crate::proxy::states::ZStream;
use crate::repeater::conn::RepeaterConn;
use crate::repeater::msg_from_ui::SendInfo;

// Convert SendInfo -> RepeaterConn<ZStream>
impl TryFrom<SendInfo> for RepeaterConn<ZStream> {
    type Error = AddressError;

    fn try_from(info: SendInfo) -> Result<Self, Self::Error> {
        // default update to true if not set in info.update;
        let update = info.update.unwrap_or(true);
        let path = PathBuf::from(&info.file);
        let server_info = ServerInfo::try_from(info.server_info)?;
        Ok(RepeaterConn {
            server_info,
            stream: ZStream,
            path,
            update,
        })
    }
}
