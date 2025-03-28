/* Description:
 *      Trait to check if http/ws request/response should be intercepted.
 *
 *      For http,
 *          1. request, Some(true)
 *          2. response, oneonestruct.need_response, set in update_resume_info
 *             state's need_response var set by interceptor ui.
 *
 *      For ws,
 *          1. request, Some(true).
 *          2. response, needs to query commander so always None
 */

pub trait ShouldIntercept {
    fn should_intercept(&self) -> Option<bool>;
}
