use super::WsStruct;
use crate::interceptor::message::to_ui::InterToUI;
use crate::proxy::handler_state::Intercept;

impl<T, E> Intercept for WsStruct<T, E> {
    fn get_inter_info(&self) -> InterToUI {
        InterToUI::build_ws(
            self.http_id,
            self.log_id,
            &self.role,
            self.frame.as_ref().unwrap().is_binary(),
        )
    }
}
