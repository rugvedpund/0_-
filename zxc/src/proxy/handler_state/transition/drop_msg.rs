use crate::proxy::handler_state::error::ProxyStateError;

// Trait to decide to continue on drop when the message is dropped by
// interceptor
pub trait DropMsg {
    fn continue_on_drop() -> Result<(), ProxyStateError>;
}
