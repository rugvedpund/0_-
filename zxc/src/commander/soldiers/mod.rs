use http::http_comm_storage::HttpCommStorage;
use tokio::sync::mpsc::{Receiver, Sender};
use ws::ws_comm_storage::WsCommStorage;

use super::{CommanderResponse, CommunicateError, WS_REGISTER};
use crate::file_types::FileType;
use crate::id::Id;
use crate::proxy::handler_state::role::Role;
pub mod http;
pub mod ws;

const BROADCAST_NONE: &str = "Broadcast None";

#[derive(Default)]
pub struct Soldiers {
    http: HttpCommStorage,
    ws: WsCommStorage,
}

impl Soldiers {
    pub fn add_http_handle(
        &mut self,
        id: usize,
    ) -> Receiver<CommanderResponse> {
        self.http.add_handle(id)
    }

    pub async fn send_ws_log(
        &mut self,
        id: usize,
        role: Role,
    ) -> Result<(), CommunicateError> {
        self.ws.send_log(id, role).await
    }

    pub async fn ws_send_should_intercept_response(
        &mut self,
        id: usize,
    ) -> Result<(), CommunicateError> {
        self.ws
            .send_should_intercept_response(id)
            .await
    }

    pub fn pop_http_sender(
        &mut self,
        id: usize,
    ) -> Result<Sender<CommanderResponse>, CommunicateError> {
        self.http
            .pop_sender(id)
            .ok_or(CommunicateError::NoId(id, WS_REGISTER))
    }

    pub fn add_ws_handle(
        &mut self,
        id: usize,
        client: Sender<CommanderResponse>,
        server: Sender<CommanderResponse>,
    ) {
        self.ws.add_handle(id, client, server);
    }

    pub async fn send_response_ft(
        &mut self,
        id: usize,
        ft: &FileType,
        res: CommanderResponse,
    ) -> Result<(), CommunicateError> {
        match ft {
            FileType::Req | FileType::Res => {
                send_response(id, self.http.iter(), res).await
            }
            FileType::Wreq => {
                let iter = self.ws.iter(Role::Server);
                send_response(id, iter, res).await
            }
            FileType::Wres => {
                let iter = self.ws.iter(Role::Client);
                send_response(id, iter, res).await
            }
        }
    }

    pub fn remove_from_http_store(&mut self, id: usize) -> bool {
        remove_from_store(self.http.store_as_mut(), id)
    }

    pub fn remove_from_ws_store(&mut self, id: usize) -> bool {
        remove_from_store(self.ws.store_as_mut(), id)
    }

    pub fn set_ws_need_response(
        &mut self,
        id: usize,
    ) -> Result<(), CommunicateError> {
        self.ws.set_need_response(id)
    }

    pub async fn broadcast_none_http(
        &mut self,
        list: &mut Vec<(usize, usize)>,
    ) -> Result<(), CommunicateError> {
        broadcast_none(list, self.http.iter()).await
    }

    pub async fn broadcast_none_wreq(
        &mut self,
        list: &mut Vec<(usize, usize)>,
    ) -> Result<(), CommunicateError> {
        broadcast_none(list, self.ws.iter(Role::Server)).await
    }

    pub async fn broadcast_none_wres(
        &mut self,
        list: &mut Vec<(usize, usize)>,
    ) -> Result<(), CommunicateError> {
        broadcast_none(list, self.ws.iter(Role::Client)).await
    }
}

pub fn remove_from_store<T>(list: &mut Vec<T>, id: usize) -> bool
where
    T: Id,
{
    if let Some(index) = list
        .iter()
        .position(|comm| comm.id() == id)
    {
        list.swap_remove(index);
        return true;
    }
    false
}

pub async fn broadcast_none<'a, T>(
    list: &mut Vec<(usize, usize)>,
    mut senders: T,
) -> Result<(), CommunicateError>
where
    T: Iterator<Item = (usize, &'a Sender<CommanderResponse>)>,
{
    if !list.is_empty() {
        for (id, _) in list.iter() {
            senders
                .find(|(sid, _)| sid == id)
                .ok_or(CommunicateError::NoId(*id, BROADCAST_NONE))?
                .1
                .send(CommanderResponse::Resume(None))
                .await?;
        }
        list.clear();
    }
    Ok(())
}

pub async fn send_response<'a, T>(
    to_send: usize,
    mut senders: T,
    response: CommanderResponse,
) -> Result<(), CommunicateError>
where
    T: Iterator<Item = (usize, &'a Sender<CommanderResponse>)>,
{
    match senders.find(|(sid, _)| sid == &to_send) {
        Some((_, sender)) => Ok(sender.send(response).await?),
        None => Err(CommunicateError::ResponseNoId(to_send, response)),
    }
}
