use bytes::BytesMut;
use tokio_tungstenite::tungstenite::Message;

use super::RWebSocket;
use crate::proxy::handler_state::transition::update_frame::error::ProxyUpdateFrameError;
use crate::repeater::states::transition::bytes_to_frame::RepeaterBytesToFrame;

impl<T> RepeaterBytesToFrame for RWebSocket<T> {
    type Frame = Message;

    fn parse_frame(
        &mut self,
        buf: BytesMut,
    ) -> Result<Self::Frame, ProxyUpdateFrameError> {
        let msg = Message::Text(
            String::from_utf8_lossy(&buf)
                .to_string()
                .into(),
        );
        self.data = Some(buf);
        Ok(msg)
    }

    fn frame_to_payload(&mut self, payload: Self::Frame) {
        self.frame = Some(payload);
    }
}
