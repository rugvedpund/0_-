use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UnixSockError {
    #[error("accept| {0}")]
    Accept(io::Error),
    #[error("closed")]
    Closed,
    #[error("EOF")]
    EoF,
    #[error("read| {0}")]
    Read(io::Error),
    #[error("readable| {0}")]
    Readable(io::Error),
    #[error("write| {0}")]
    Write(io::Error),
    #[error("block")]
    Block,
}
