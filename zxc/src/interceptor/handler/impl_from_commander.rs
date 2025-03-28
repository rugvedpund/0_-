use super::InterceptorHandler;
use crate::interceptor::message::to_ui::InterToUI;
use crate::run::boundary::FromCommander;

impl FromCommander for InterceptorHandler {
    type Message = InterToUI;

    async fn recv(&mut self) -> Option<Self::Message> {
        self.from_commander.recv().await
    }
}
