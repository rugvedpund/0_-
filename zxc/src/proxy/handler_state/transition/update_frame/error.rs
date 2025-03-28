use oneone::UpdateFrameError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProxyUpdateFrameError {
    #[error("updating frame")]
    HttpFrame(#[from] UpdateFrameError),
    #[error("invalid ws frame")]
    InvalidWsFrame,
}
