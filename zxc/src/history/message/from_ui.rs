use serde::Deserialize;

use crate::forward_info::ForwardInfo;

// Messages from history UI
#[derive(Debug, Deserialize)]
pub struct HistoryUImsg {
    _id: u8,
    pub operation: HistoryUIOps,
}

// History UI Operations
#[derive(Deserialize, Debug)]
pub enum HistoryUIOps {
    Close,
    Forward(ForwardInfo),
    ReloadConfig,
}

impl From<HistoryUImsg> for HistoryUIOps {
    fn from(msg: HistoryUImsg) -> Self {
        msg.operation
    }
}
