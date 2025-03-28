use crate::interceptor::message::from_ui::resume_info::ResumeInfo;

/* Description:
*      Trait to check if http request/response should be rewritten.
*      http only.
*
*      For ws, already rewritten by ui and no additional data is added. So,
*      always false.
*/

pub trait ShouldRewrite {
    fn should_rewrite(&self, resume_info: &ResumeInfo) -> bool;
}
