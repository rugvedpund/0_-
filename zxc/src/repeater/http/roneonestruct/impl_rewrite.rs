use std::path::PathBuf;

use tokio::fs::File;

use super::Roneone;
use crate::repeater::states::transition::rewrite::{Newrite, Rewrite};

impl<T> Rewrite for Roneone<T> {
    // If frame was uploaded, then rewrite
    #[inline]
    fn should_rewrite(&self) -> bool {
        self.update
    }
    fn get_write_data_and_file(&mut self) -> (&[u8], &mut File) {
        // safe to unwrap,
        // since data is set in read_from_file / update frame state
        (self.payload.as_ref().unwrap(), &mut self.file)
    }
}

const NEWRITE_ERROR: &str = "No new Write for http";
impl<T> Newrite for Roneone<T> {
    fn data_as_ref(&self) -> &[u8] {
        panic!("{}", NEWRITE_ERROR)
    }
    fn path_as_ref(&self) -> &PathBuf {
        panic!("{}", NEWRITE_ERROR)
    }

    fn update_path(&mut self) {
        panic!("{}", NEWRITE_ERROR)
    }
}
