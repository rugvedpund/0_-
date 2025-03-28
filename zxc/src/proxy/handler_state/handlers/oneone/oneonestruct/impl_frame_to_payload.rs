use oneone::InfoLine;
use protocol_traits::Frame;

use super::OneOneStruct;
use crate::proxy::handler_state::FrameToPayload;

impl<T, E, U> FrameToPayload for OneOneStruct<T, E, U>
where
    U: InfoLine,
{
    fn frame_to_payload(&mut self) {
        // safe to unwrap
        self.payload = Some(self.frame.take().unwrap().into_data());
    }
}
