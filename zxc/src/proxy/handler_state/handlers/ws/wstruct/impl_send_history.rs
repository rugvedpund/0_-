use tokio::sync::mpsc::Sender;

use super::WsStruct;
use crate::history::message::from_commander::CommanderToHistory;
use crate::proxy::handler_state::transition::write_history::SendHistory;

impl<T, E> SendHistory for WsStruct<T, E> {
    fn get_sender(&self) -> &Sender<CommanderToHistory> {
        &self.history_sendr
    }
}
