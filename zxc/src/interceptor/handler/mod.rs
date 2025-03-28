use std::fmt::Debug;

use bytes::BytesMut;
use tokio::net::UnixStream;
use tokio::sync::mpsc::{Receiver, Sender};
use zxc_derive::{Buffer, CloseAction, FlushStorage};

use super::message::from_ui::InterUIOps;
use super::message::to_ui::InterToUI;
use crate::CAPACITY_2MB;
use crate::io::unix_sock::error::UnixSockError;
use crate::run::boundary::{Buffer, CloseAction, FlushStorage};
mod impl_from_commander;
mod impl_handle_commander_msg;
mod impl_handle_ui;
mod impl_notify_commander;

// Handles the Interceptor side of proxy
#[derive(Buffer, FlushStorage, CloseAction)]
pub struct InterceptorHandler {
    to_commander: Sender<InterUIOps>,
    from_commander: Receiver<InterToUI>,
    intercept_state: bool,
    buf: BytesMut,
}

impl InterceptorHandler {
    #[inline(always)]
    pub fn new(
        to_commander: Sender<InterUIOps>,
        from_commander: Receiver<InterToUI>,
    ) -> Self {
        Self {
            to_commander,
            from_commander,
            intercept_state: false,
            buf: BytesMut::with_capacity(CAPACITY_2MB),
        }
    }
}

impl Debug for InterceptorHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "interceptor")
    }
}
