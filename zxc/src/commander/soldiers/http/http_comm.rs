use tokio::sync::mpsc::Sender;
use zxc_derive::Id;

use crate::commander::communicate::response::CommanderResponse;
use crate::id::Id;

// Struct for storing http soldier communication
#[derive(Id)]
pub struct HttpComm {
    id: usize,
    sender: Sender<CommanderResponse>,
}

impl HttpComm {
    pub fn new(id: usize, sender: Sender<CommanderResponse>) -> HttpComm {
        HttpComm {
            id,
            sender,
        }
    }

    pub fn into_sender(self) -> Sender<CommanderResponse> {
        self.sender
    }

    pub fn sender(&self) -> &Sender<CommanderResponse> {
        &self.sender
    }
}
