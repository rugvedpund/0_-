use std::path::PathBuf;

use bytes::BytesMut;
use tokio::fs::File;
use zxc_derive::RepeaterReadFile;

use crate::repeater::states::transition::read_from_file::RepeaterReadFile;

mod conversion;
mod impl_add_raw;
mod impl_read_write;
mod impl_repeater_bytes_to_frame;
mod impl_rewrite;
mod impl_should_update;
mod impl_write_response;

// repeater http handler struct
#[derive(RepeaterReadFile)]
pub struct Roneone<T> {
    buf: BytesMut,
    path: PathBuf,
    stream: T,
    file: File,
    payload: Option<BytesMut>,
    update: bool,
}

impl<T> Roneone<T> {
    pub fn get_payload(&mut self) -> Option<BytesMut> {
        self.payload.take()
    }
}
