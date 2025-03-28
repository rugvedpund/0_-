use std::path::PathBuf;

use tokio::fs::File;

use super::RWebSocket;
use crate::file_types::EXT_WREQ;
use crate::repeater::states::transition::rewrite::{Newrite, Rewrite};

impl<T> Rewrite for RWebSocket<T> {
    #[inline(always)]
    fn should_rewrite(&self) -> bool {
        false
    }

    // No rewrite for ws, so always return data None
    fn get_write_data_and_file(&mut self) -> (&[u8], &mut File) {
        panic!("No rewrite for ws");
    }
}

/* Steps:
 *        1. update path to id.wreq
 *        2. increment id
 */

impl<T> Newrite for RWebSocket<T> {
    fn update_path(&mut self) {
        self.path
            .set_file_name(self.log_id.to_string());
        self.path.set_extension(EXT_WREQ);
    }

    fn data_as_ref(&self) -> &[u8] {
        self.data.as_ref().unwrap()
    }

    fn path_as_ref(&self) -> &PathBuf {
        &self.path
    }
}
