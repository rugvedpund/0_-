use bytes::BytesMut;
use tokio::fs::File;

use super::Roneone;
use crate::CAPACITY_2MB;
use crate::repeater::conn::RepeaterConn;

// Convert (RepeaterConn<T>, File) -> Roneone

impl<T> From<(RepeaterConn<T>, File)> for Roneone<T> {
    fn from((conn, file): (RepeaterConn<T>, File)) -> Self {
        Self {
            buf: BytesMut::with_capacity(CAPACITY_2MB),
            path: conn.path,
            stream: conn.stream,
            file,
            payload: None,
            update: conn.update,
        }
    }
}
