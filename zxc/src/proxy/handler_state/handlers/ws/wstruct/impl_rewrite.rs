use tokio::fs::File;

use super::WsStruct;
use crate::proxy::handler_state::transition::rewrite::Rewrite;

impl<T, E> Rewrite for WsStruct<T, E> {
    #[inline(always)]
    fn file_and_data(&mut self) -> (&mut File, &[u8]) {
        panic!("No rewrite for WsHandle");
    }
}
