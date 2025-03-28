use crate::interceptor::message::from_ui::resume_info::ResumeInfo;

/* Description:
 *      Update whether the response needs to be intercepted updated (http only)
 */

pub trait UpdateResumeInfo {
    fn update_resume_info(&mut self, resume_info: &ResumeInfo);
}
