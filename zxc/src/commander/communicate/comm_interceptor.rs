use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::commander::CommunicateError;
use crate::file_types::FileType;
use crate::interceptor::message::from_ui::InterUIOps;
use crate::interceptor::message::to_ui::InterToUI;

// Type Alias

// Handles Commander to interceptor communication
pub struct InterceptorComm {
    pub from_interceptor: Receiver<InterUIOps>,
    to_interceptor: Sender<InterToUI>,
    http_queue: Vec<(usize, usize)>,
    wreq_queue: Vec<(usize, usize)>,
    wres_queue: Vec<(usize, usize)>,
    status: bool,
}

impl InterceptorComm {
    #[inline(always)]
    pub fn new(
        from_interceptor: Receiver<InterUIOps>,
        to_interceptor: Sender<InterToUI>,
    ) -> Self {
        Self {
            from_interceptor,
            to_interceptor,
            http_queue: Vec::new(),
            wreq_queue: Vec::new(),
            wres_queue: Vec::new(),
            status: false,
        }
    }

    pub fn status(&self) -> bool {
        self.status
    }

    pub fn toggle(&mut self) {
        self.status = !self.status;
    }

    pub async fn send_to_interceptor(
        &mut self,
        conn_id: usize,
        msg: InterToUI,
    ) -> Result<(), SendError<InterToUI>> {
        let queue = if msg.is_http() {
            &mut self.http_queue
        } else if msg.is_wreq() {
            &mut self.wreq_queue
        } else {
            &mut self.wres_queue
        };
        let log_id = msg.id();
        queue.push((conn_id, log_id));
        self.to_interceptor.send(msg).await
    }

    pub fn http_queue_as_mut(&mut self) -> &mut Vec<(usize, usize)> {
        &mut self.http_queue
    }

    pub fn wreq_queue_as_mut(&mut self) -> &mut Vec<(usize, usize)> {
        &mut self.wreq_queue
    }

    pub fn wres_queue_as_mut(&mut self) -> &mut Vec<(usize, usize)> {
        &mut self.wres_queue
    }

    pub fn conn_id_from_log_id(
        &mut self,
        log_id: usize,
        ft: &FileType,
    ) -> Result<usize, CommunicateError> {
        let queue = match ft {
            FileType::Req | FileType::Res => &mut self.http_queue,
            FileType::Wreq => &mut self.wreq_queue,
            FileType::Wres => &mut self.wres_queue,
        };
        let pos = queue
            .iter()
            .position(|&(_, stored_log_id)| stored_log_id == log_id)
            .ok_or(CommunicateError::NoId(log_id, "resume_intercept"))?;
        let conn_id = queue.swap_remove(pos).0;
        Ok(conn_id)
    }
}
