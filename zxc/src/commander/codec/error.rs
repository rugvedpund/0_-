use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("Base64| {0}")]
    Base64(#[from] base64::EncodeSliceError),
    #[error("Url| {0}")]
    Url(#[from] std::str::Utf8Error),
}

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Base64| {0}")]
    Base64(#[from] base64::DecodeSliceError),
    #[error("Url| {0}")]
    Url(#[from] std::str::Utf8Error),
}

#[derive(Error, Debug)]
pub enum CodecError {
    #[error("Encode| {0}")]
    Encode(#[from] EncodeError),
    #[error("Decode| {0}")]
    Decode(#[from] DecodeError),
}
