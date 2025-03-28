use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

use super::handlers::error::WsError;
use super::handlers::oneonestruct::OneOneRWError;
use super::transition::update_frame::error::ProxyUpdateFrameError;
use crate::commander::CommanderRequest;
use crate::commander::communicate::response::convert::WrongMessage;
use crate::history::message::from_commander::CommanderToHistory;
use crate::io::file::FileErrorInfo;
use crate::io::socket::ConnectError;
use crate::proxy::states::connection::encrypt::ServerEncryptError;

// Various Errors that can occur on handler state transition
#[derive(Error, Debug)]
pub enum ProxyStateError {
    // Communication Error
    #[error("Commander Request| {0}")]
    CommanderRequest(#[from] SendError<CommanderRequest>),
    #[error("Commander Response| {0}")]
    CommanderResponse(&'static str),
    #[error("wrong command| {0}")]
    WrongMessage(#[from] WrongMessage),

    // ----- Read Write -----
    // HTTP
    #[error("HTTP Error| {0}")]
    OneOne(#[from] OneOneRWError),
    // WebSocket
    #[error("Ws| {0}")]
    Ws(#[from] WsError),

    // ----- Write History -----
    #[error("Serialize| {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("ws History Send")]
    HistorySend(#[from] SendError<CommanderToHistory>),

    // ----- File Ops -----
    #[error("file| {0}")]
    FileIo(#[from] FileErrorInfo),
    #[error("rewrite file| {0}")]
    ReWriteFile(FileErrorInfo),

    // ----- Should Intercept -----
    #[error("Ws Should Intercept")]
    WsShouldIntercept,

    // ----- Update Frame -----
    #[error("Update Frame| {0}")]
    UpdateFrame(#[from] ProxyUpdateFrameError),

    // ----- NewConnection -----
    #[error("Reconnect| {0}")]
    Reconnect(#[from] ConnectError),

    #[error("Server Encrypt| {0}")]
    ServerEncrypt(#[from] ServerEncryptError),
    #[error("msg drop")]
    Drop,
}
