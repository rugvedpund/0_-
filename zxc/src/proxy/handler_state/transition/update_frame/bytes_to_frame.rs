use bytes::BytesMut;

use super::error::ProxyUpdateFrameError;

/* Description:
 *      Trait to parse bytes to frame.
 */

pub trait BytesToFrame {
    type Frame;

    fn parse_frame(
        &self,
        buf: BytesMut,
    ) -> Result<Self::Frame, ProxyUpdateFrameError>;

    fn add_frame(&mut self, frame: Self::Frame);
}
