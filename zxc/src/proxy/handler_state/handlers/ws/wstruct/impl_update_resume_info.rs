use crate::{
    interceptor::message::from_ui::resume_info::ResumeInfo,
    proxy::handler_state::transition::resume_intercept::update_resume_info::UpdateResumeInfo,
};

use super::WsStruct;

// Blanket implementation
impl<T, E> UpdateResumeInfo for WsStruct<T, E> {
    #[inline(always)]
    fn update_resume_info(&mut self, _: &ResumeInfo) {}
}
