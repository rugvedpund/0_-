use super::ws_register::HistoryWsRegisterInfo;

// Messages from commander to history
pub enum CommanderToHistory {
    // From Commander - http
    Http(String),
    // From Soldiers - ws
    RegisterWs(HistoryWsRegisterInfo),
    WebSocket(usize, String),
    RemoveWs(usize),
}
