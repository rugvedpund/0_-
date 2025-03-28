use bytes::BytesMut;
use oneone::{OneOne, Request, UpdateHttp};
use protocol_traits::Frame;

use super::Roneone;
use crate::proxy::handler_state::transition::update_frame::error::ProxyUpdateFrameError;
use crate::repeater::states::transition::bytes_to_frame::RepeaterBytesToFrame;

impl<T> RepeaterBytesToFrame for Roneone<T> {
    type Frame = OneOne<Request>;

    fn parse_frame(
        &mut self,
        buf: BytesMut,
    ) -> Result<Self::Frame, ProxyUpdateFrameError> {
        Ok(OneOne::<Request>::update(buf)?)
    }

    fn frame_to_payload(&mut self, frame: Self::Frame) {
        self.payload = Some(frame.into_data())
    }
}
