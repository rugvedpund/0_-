use oneone::InfoLine;
use tokio::fs::File;

use super::OneOneStruct;
use crate::proxy::handler_state::transition::rewrite::Rewrite;

impl<T, E, U> Rewrite for OneOneStruct<T, E, U>
where
    U: InfoLine,
{
    fn file_and_data(&mut self) -> (&mut File, &[u8]) {
        (self.file.as_mut().unwrap(), self.payload.as_ref().unwrap()) // safe to unwrap
    }
}
