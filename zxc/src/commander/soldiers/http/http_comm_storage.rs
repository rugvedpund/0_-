use tokio::sync::mpsc::{
    Receiver, Sender, {self}
};

use super::http_comm::HttpComm;
use crate::commander::communicate::response::CommanderResponse;
use crate::id::Id;

// Stores Vector of HttpComm
#[derive(Default)]
pub struct HttpCommStorage {
    store: Vec<HttpComm>,
}

impl HttpCommStorage {
    pub fn add_handle(&mut self, id: usize) -> Receiver<CommanderResponse> {
        let (tx, rx) = mpsc::channel::<CommanderResponse>(1);
        self.store.push(HttpComm::new(id, tx));
        rx
    }

    pub fn pop_sender(
        &mut self,
        id: usize,
    ) -> Option<Sender<CommanderResponse>> {
        self.store
            .iter()
            .position(|x| x.id() == id)
            .map(|pos| {
                self.store
                    .swap_remove(pos)
                    .into_sender()
            })
    }

    pub fn store_as_mut(&mut self) -> &mut Vec<HttpComm> {
        &mut self.store
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (usize, &Sender<CommanderResponse>)> {
        self.store
            .iter()
            .map(|x| (x.id(), x.sender()))
    }
}
