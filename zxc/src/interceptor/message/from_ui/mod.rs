use serde::Deserialize;

use crate::commander::codec::Codec;
use crate::file_types::FileType;
use crate::forward_info::ForwardInfo;
pub mod ftspec;
pub mod resume_info;
use ftspec::*;
use resume_info::*;

// Message from UI
#[derive(Deserialize, Debug)]
pub struct InterUImsg {
    pub id: usize,
    pub operation: InterUIOps,
}

impl InterUImsg {
    pub fn op(&self) -> &InterUIOps {
        &self.operation
    }
}

impl From<InterUImsg> for InterUIOps {
    fn from(msg: InterUImsg) -> Self {
        msg.operation
    }
}

// Different types of operation from UI
#[derive(Debug, Deserialize)]
pub enum InterUIOps {
    Close,
    Drop(usize, FileType),
    Resume(ResumeInfo),
    Encode {
        codec: Codec,
        data: String,
    },

    Decode {
        codec: Codec,
        data: String,
    },
    Forward(ForwardInfo),
    Toggle,
}
