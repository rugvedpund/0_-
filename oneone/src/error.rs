use std::fmt::Debug;

use thiserror::Error;

use crate::InfoLineError;
use crate::state::body_reader::chunked_reader::ChunkReaderError;

#[derive(Debug, Error)]
pub enum HttpReadError {
    #[error("infoline| {0}")]
    InfoLine(#[from] InfoLineError),
    #[error("header struct| {0}")]
    HeaderStruct(String),
    #[error("chunkreader| {0}")]
    ChunkReaderFailed(#[from] ChunkReaderError),
    // Not enough data
    #[error("chunk reader not enough data")]
    ChunkReaderNotEnoughData,
    #[error("header not enough data")]
    HeaderNotEnoughData,
}
