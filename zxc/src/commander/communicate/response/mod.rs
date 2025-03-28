use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use std::sync::Arc;
pub mod convert;

use tokio::sync::mpsc::{Receiver, Sender};
use tokio_rustls::TlsConnector;
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::rustls::client::WebPkiServerVerifier;

use crate::commander::captain_crypto::error::CertError;
use crate::history::message::from_commander::CommanderToHistory;
use crate::interceptor::message::from_ui::resume_info::ResumeInfo;

pub enum CommanderResponse {
    ClientConfig(Arc<TlsConnector>),
    Verifier(Arc<WebPkiServerVerifier>),
    ServerConfig(Option<Arc<ServerConfig>>),
    NewCertificate(Result<Arc<ServerConfig>, CertError>),
    HttpLog(Option<(usize, PathBuf, Sender<CommanderToHistory>)>),
    Resume(Option<ResumeInfo>),
    WsProxyReply(bool),
    WsRegisterReply((Receiver<CommanderResponse>, Sender<CommanderToHistory>)),
    WsLog(usize),
    WsInterceptReply(bool),
    Drop,
}

impl CommanderResponse {
    pub fn is_drop_msg(&self) -> bool {
        matches!(self, CommanderResponse::Drop)
    }

    pub fn wreq_need_response(&self) -> bool {
        if let CommanderResponse::Resume(Some(info)) = self {
            return info.is_wreq() && info.need_response();
        }
        false
    }
}

impl Display for CommanderResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommanderResponse::ClientConfig(_) => write!(f, "ClientConfig"),
            CommanderResponse::Verifier(_) => write!(f, "Verifier"),
            CommanderResponse::ServerConfig(_) => write!(f, "ServerConfig"),
            CommanderResponse::NewCertificate(_) => {
                write!(f, "NewCertificate")
            }
            CommanderResponse::HttpLog(_) => write!(f, "HttpLog"),
            CommanderResponse::Resume(_) => write!(f, "Resume"),
            CommanderResponse::WsProxyReply(_) => write!(f, "WsProxyReply"),
            CommanderResponse::WsRegisterReply(..) => {
                write!(f, "WsRegisterReply")
            }
            CommanderResponse::WsLog(_) => write!(f, "WsLog"),
            CommanderResponse::WsInterceptReply(_) => write!(f, "WsIntercept"),
            CommanderResponse::Drop => write!(f, "Drop"),
        }
    }
}

impl Debug for CommanderResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
