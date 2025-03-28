use bytes::BytesMut;
use oneone::InfoLine;

use super::OneOneStruct;
use crate::proxy::handler_state::transition::read_modified_file::add_raw::AddRaw;

impl<T, E, U> AddRaw for OneOneStruct<T, E, U>
where
    U: InfoLine,
{
    fn add_raw(&mut self, buf: BytesMut) {
        self.payload = Some(buf);
    }
}
