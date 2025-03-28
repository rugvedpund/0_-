use oneone::InfoLine;
use tokio::sync::mpsc::Sender;

use super::OneOneStruct;
use crate::history::message::from_commander::CommanderToHistory;
use crate::proxy::handler_state::transition::write_history::SendHistory;

impl<T, E, U> SendHistory for OneOneStruct<T, E, U>
where
    U: InfoLine,
{
    fn get_sender(&self) -> &Sender<CommanderToHistory> {
        self.history_sendr.as_ref().unwrap()
    }
}
