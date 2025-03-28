use bytes::BytesMut;

use crate::proxy::handler_state::transition::update_frame::error::ProxyUpdateFrameError;

/* Description:
 *     Trait to convert Bytes to Frame (http/ws).
 *
 * Transition:
 *     UpdateFrame -> ReWrite
 */

pub trait RepeaterBytesToFrame {
    type Frame;

    // Convert buf to Frame
    fn parse_frame(
        &mut self,
        buf: BytesMut,
    ) -> Result<Self::Frame, ProxyUpdateFrameError>;

    // Convert Frame to payload
    fn frame_to_payload(&mut self, frame: Self::Frame);
}
