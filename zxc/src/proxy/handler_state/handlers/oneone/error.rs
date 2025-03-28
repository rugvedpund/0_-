use std::fmt::Debug;

use oneone::{Request, Response};
use thiserror::Error;

use super::{OneOneStruct, StatusCodeError};
use crate::proxy::handler_state::error::ProxyStateError;
use crate::proxy::server_info::json::ServerInfoJson;

// one one errors

#[derive(Error, Debug)]
pub enum HandleOneOneError<T, E> {
    // client/server state
    #[error("proxy state error| {0}")]
    ProxyError(#[from] ProxyStateError),

    // ----- client state => client conn
    #[error("http read")]
    SendToServer(OneOneStruct<T, E, Request>, ProxyStateError),
    #[error("new connection")]
    NeedNewConnection(OneOneStruct<T, E, Request>, ServerInfoJson),

    // server state
    #[error("http read")]
    ReadFromServer(OneOneStruct<E, T, Response>, ProxyStateError),
    // Misc
    #[error("status code| {0}")]
    StatusCode(#[from] StatusCodeError),
}
