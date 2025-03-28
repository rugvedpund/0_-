use std::path::PathBuf;

use oneone::InfoLine;

use super::OneOneStruct;
use crate::proxy::handler_state::transition::write_log::log::Log;

impl<T, E, U> Log for OneOneStruct<T, E, U>
where
    U: InfoLine,
{
    fn path(&self) -> &PathBuf {
        self.path.as_ref().unwrap() // safe to unwrap
    }

    fn log_data(&self) -> &[u8] {
        self.payload.as_ref().unwrap() // safe to unwrap
    }
}
