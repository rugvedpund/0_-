use super::WsStruct;
use crate::commander::communicate::response::convert::WrongMessage;
use crate::commander::{CommanderRequest, CommanderResponse};
use crate::proxy::handler_state::ShouldLog;

impl<T, E> ShouldLog for WsStruct<T, E> {
    type LogResult = usize;

    /* Steps:
     *      if frame is text || binary , return Some(CommanderRequest::WsLog(id, role))
     */

    fn get_log_request(&self) -> Option<CommanderRequest> {
        // safe to unwrap
        if self.frame.as_ref().unwrap().is_text()
            || self.frame.as_ref().unwrap().is_binary()
        {
            return Some(CommanderRequest::WsLog(self.id, self.role));
        }
        None
    }

    /* Steps:
     *      Use TryFrom trait implementation in communicate/response/convert.rs
     *      to convert CommanderResponse::WsLog to Option<usize>
     */

    #[inline(always)]
    fn parse_log_response(
        &self,
        response: CommanderResponse,
    ) -> Result<Option<usize>, WrongMessage> {
        Option::<usize>::try_from(response)
    }

    #[inline(always)]
    fn update_path(&mut self, id: Self::LogResult) {
        self.log_id = id;
    }
}
