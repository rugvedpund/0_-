use super::WsStruct;
use crate::interceptor::message::from_ui::resume_info::ResumeInfo;
use crate::proxy::handler_state::transition::update_frame::should_rewrite::ShouldRewrite;

/* Steps:
 *      Always return false as no additional data is added.
 */

impl<T, E> ShouldRewrite for WsStruct<T, E> {
    #[inline(always)]
    fn should_rewrite(&self, _: &ResumeInfo) -> bool {
        false
    }
}
