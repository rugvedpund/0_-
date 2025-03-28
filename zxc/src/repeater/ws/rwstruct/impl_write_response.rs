use std::path::PathBuf;

use tokio_tungstenite::tungstenite::Message;

use super::RWebSocket;
use crate::file_types::EXT_WRES;
use crate::repeater::states::transition::write_response::WriteResponse;

impl<T> WriteResponse for RWebSocket<T> {
    // filename: log_id.wres
    fn update_response_path(&mut self) {
        self.path
            .set_file_name(self.log_id.to_string());
        self.path.set_extension(EXT_WRES);
    }

    fn response_path(&self) -> &PathBuf {
        &self.path
    }

    fn response_data(&self) -> &[u8] {
        // safe to unwrap
        match self.frame.as_ref().unwrap() {
            Message::Text(frame) => frame.as_bytes(),
            Message::Binary(frame) => frame,
            _ => &[],
        }
    }
}
