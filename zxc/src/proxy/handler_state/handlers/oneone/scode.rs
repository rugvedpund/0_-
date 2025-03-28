use std::str;

use bytes::BytesMut;
use oneone::{InfoLine, InfoLineError, Response};
use thiserror::Error;

/* Description:
 *      Get status code from raw response
 *
 * Args:
 *      data: BytesMut
 *
 * Steps:
 *      1. call split_infoline() with data
 *      2. str() -> parse -> u16
 *
 * Returns:
 *      Ok(u8)
 *
 * Error:
 *      StatusCodeError::InfoLine [1]
 *      StatusCodeError::Utf8 [2]
 *      StatusCodeError::ParseInt [2]
 */

#[derive(Error, Debug)]
pub enum StatusCodeError {
    // InfoLine Error
    #[error("{0}")]
    InfoLine(#[from] InfoLineError),
    // Utf8 Error
    #[error("Not valid utf8| {0}")]
    Utf8(#[from] std::str::Utf8Error),
    // Parse int
    #[error("Parse int| {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}

pub fn get_status_code(data: BytesMut) -> Result<u16, StatusCodeError> {
    let resp = Response::build_infoline(data)?;
    let scode = resp.status();
    Ok(str::from_utf8(scode)?.parse::<u16>()?)
}
