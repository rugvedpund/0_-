use bytes::BytesMut;
use oneone::{InfoLine, OneOne, UpdateHttp};

use super::OneOneStruct;
use crate::proxy::handler_state::transition::update_frame::bytes_to_frame::BytesToFrame;
use crate::proxy::handler_state::transition::update_frame::error::ProxyUpdateFrameError;

/* Errors:
 *      ProxyUpdateFrameError::HttpFrame
 */

impl<T, E, U> BytesToFrame for OneOneStruct<T, E, U>
where
    U: InfoLine,
    OneOne<U>: UpdateHttp,
{
    type Frame = OneOne<U>;

    fn parse_frame(
        &self,
        buf: BytesMut,
    ) -> Result<Self::Frame, ProxyUpdateFrameError> {
        Ok(OneOne::<U>::update(buf)?)
    }

    fn add_frame(&mut self, frame: Self::Frame) {
        self.frame = Some(frame);
    }
}
