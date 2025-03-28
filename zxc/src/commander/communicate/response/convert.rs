use thiserror::Error;

use super::*;

#[derive(Debug, Error)]
#[error("wrong response| {0}")]
pub struct WrongMessage(String);

// Various TryFrom implementations to get associated values from
// CommanderResponse
impl TryFrom<CommanderResponse> for Arc<TlsConnector> {
    type Error = WrongMessage;

    fn try_from(value: CommanderResponse) -> Result<Self, Self::Error> {
        match value {
            CommanderResponse::ClientConfig(config) => Ok(config),
            _ => Err(WrongMessage(value.to_string())),
        }
    }
}

impl TryFrom<CommanderResponse> for Arc<WebPkiServerVerifier> {
    type Error = WrongMessage;

    fn try_from(value: CommanderResponse) -> Result<Self, Self::Error> {
        match value {
            CommanderResponse::Verifier(verifier) => Ok(verifier),
            _ => Err(WrongMessage(value.to_string())),
        }
    }
}

impl TryFrom<CommanderResponse> for Option<Arc<ServerConfig>> {
    type Error = WrongMessage;

    fn try_from(value: CommanderResponse) -> Result<Self, Self::Error> {
        match value {
            CommanderResponse::ServerConfig(config) => Ok(config),
            _ => Err(WrongMessage(value.to_string())),
        }
    }
}

impl TryFrom<CommanderResponse> for Result<Arc<ServerConfig>, CertError> {
    type Error = WrongMessage;

    fn try_from(value: CommanderResponse) -> Result<Self, Self::Error> {
        match value {
            CommanderResponse::NewCertificate(config) => Ok(config),
            _ => Err(WrongMessage(value.to_string())),
        }
    }
}

impl TryFrom<CommanderResponse>
    for Option<(usize, PathBuf, Sender<CommanderToHistory>)>
{
    type Error = WrongMessage;

    fn try_from(value: CommanderResponse) -> Result<Self, Self::Error> {
        match value {
            CommanderResponse::HttpLog(config) => Ok(config),
            _ => Err(WrongMessage(value.to_string())),
        }
    }
}

impl TryFrom<CommanderResponse> for Option<ResumeInfo> {
    type Error = WrongMessage;

    fn try_from(value: CommanderResponse) -> Result<Self, Self::Error> {
        match value {
            CommanderResponse::Resume(config) => Ok(config),
            _ => Err(WrongMessage(value.to_string())),
        }
    }
}

impl TryFrom<CommanderResponse>
    for (Receiver<CommanderResponse>, Sender<CommanderToHistory>)
{
    type Error = WrongMessage;

    fn try_from(value: CommanderResponse) -> Result<Self, Self::Error> {
        match value {
            CommanderResponse::WsRegisterReply(config) => Ok(config),
            _ => Err(WrongMessage(value.to_string())),
        }
    }
}

impl TryFrom<CommanderResponse> for Option<usize> {
    type Error = WrongMessage;

    fn try_from(value: CommanderResponse) -> Result<Self, Self::Error> {
        match value {
            CommanderResponse::WsLog(config) => Ok(Some(config)),
            _ => Err(WrongMessage(value.to_string())),
        }
    }
}

impl TryFrom<CommanderResponse> for bool {
    type Error = WrongMessage;

    fn try_from(value: CommanderResponse) -> Result<Self, Self::Error> {
        match value {
            CommanderResponse::WsProxyReply(config) => Ok(config),
            _ => Err(WrongMessage(value.to_string())),
        }
    }
}
