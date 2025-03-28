use std::sync::Arc;

use bytes::BytesMut;
use oneone::{OneOne, Request};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::Sender;

use super::{ServerInfo, Tcp};
use crate::CAPACITY_2MB;
use crate::commander::CommanderRequest;
mod convert;
pub mod encrypt;

// Zero sized struct to denote no stream
pub struct ZStream;

/* Description:
 *      Initial Connection struct.
 *
 * Generics:
 *      T: Client Stream
 *      E: Server Stream
 */

pub struct Connection<T, E> {
    pub id: usize,
    pub commander: Sender<CommanderRequest>,
    pub frame: Option<OneOne<Request>>,
    pub buf: BytesMut,
    pub reader: T,
    pub writer: E,
}

impl<T, E> Connection<T, E> {
    pub fn new(
        index: usize,
        conn: T,
        tx: Sender<CommanderRequest>,
    ) -> Connection<T, ZStream> {
        Connection {
            buf: BytesMut::with_capacity(CAPACITY_2MB),
            commander: tx,
            frame: None,
            id: index,
            reader: conn,
            writer: ZStream,
        }
    }
}
