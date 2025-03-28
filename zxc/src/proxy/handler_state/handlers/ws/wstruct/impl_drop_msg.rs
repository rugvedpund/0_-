use super::{ProxyStateError, WsStruct};
use crate::proxy::handler_state::transition::drop_msg::DropMsg;

// For websocket, continue to receive message
impl<T, E> DropMsg for WsStruct<T, E> {
    #[inline(always)]
    fn continue_on_drop() -> Result<(), ProxyStateError> {
        Ok(())
    }
}
