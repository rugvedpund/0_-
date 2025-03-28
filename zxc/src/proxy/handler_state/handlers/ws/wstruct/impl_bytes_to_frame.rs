use bytes::BytesMut;
use tokio_tungstenite::tungstenite::Message;

use super::WsStruct;
use crate::proxy::handler_state::transition::update_frame::bytes_to_frame::BytesToFrame;
use crate::proxy::handler_state::transition::update_frame::error::ProxyUpdateFrameError;

/* Steps:
 *      Only text and binary frame is supported. build new frame of the same
 *      type. For other frame, return an error.
 *
 * Errors:
 *      ProxyUpdateFrameError::InvalidWsFrame
 */

impl<T, E> BytesToFrame for WsStruct<T, E> {
    type Frame = Message;

    fn parse_frame(
        &self,
        buf: BytesMut,
    ) -> Result<Self::Frame, ProxyUpdateFrameError> {
        // safe to unwrap
        let new_frame = match self.frame.as_ref().unwrap() {
            Message::Text(_) => Message::Text(
                String::from_utf8_lossy(&buf)
                    .to_string()
                    .into(),
            ),
            Message::Binary(_) => Message::Binary(buf.into()),
            _ => return Err(ProxyUpdateFrameError::InvalidWsFrame),
        };
        Ok(new_frame)
    }

    #[inline(always)]
    fn add_frame(&mut self, frame: Self::Frame) {
        self.frame = Some(frame);
    }
}
