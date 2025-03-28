use thiserror::Error;

use crate::HttpReadError;

#[derive(Debug, Error)]
pub enum UpdateFrameError {
    #[error("Failed to FindCRLF")]
    UnableToFindCRLF,
    #[error("Failed to DecodeHTTP")]
    HttpDecodeError(#[from] HttpReadError),
}
