use super::WsStruct;
use crate::proxy::handler_state::FrameToPayload;

// Blanket implementation
impl<T, E> FrameToPayload for WsStruct<T, E> {
    #[inline(always)]
    fn frame_to_payload(&mut self) {}
}
