use crate::addons::error::AddonError;
use crate::addons::handler::AddonHandler;
use crate::addons::message::from_ui::AddonMsg;
use crate::io::unix_sock::error::UnixSockError;
use crate::run::boundary::HandleUI;

// Only Close msg is supported
impl HandleUI for AddonHandler {
    async fn handle_ui(
        &mut self,
    ) -> Result<Option<(usize, String)>, AddonError> {
        let _: AddonMsg = serde_json::from_slice(&self.buf)?;
        Err(UnixSockError::Closed.into())
    }
}
