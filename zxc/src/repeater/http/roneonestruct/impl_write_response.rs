use std::path::PathBuf;

use super::Roneone;
use crate::repeater::file_builder::REPEATER_HTTP_FILENAME_RES;
use crate::repeater::states::transition::write_response::WriteResponse;

impl<T> WriteResponse for Roneone<T> {
    // rep.res
    fn update_response_path(&mut self) {
        self.path
            .set_file_name(REPEATER_HTTP_FILENAME_RES);
    }

    fn response_path(&self) -> &PathBuf {
        &self.path
    }

    // safe to unwrap
    // since, payload set in read state
    fn response_data(&self) -> &[u8] {
        self.payload.as_ref().unwrap()
    }
}
