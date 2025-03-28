use tracing::trace;

use crate::interceptor::message::from_ui::resume_info::ResumeInfo;
use crate::proxy::handler_state::ProxyState;
use crate::proxy::handler_state::error::ProxyStateError;
pub mod update_resume_info;
use update_resume_info::UpdateResumeInfo;

use super::can_communicate::CanCommunicate;

/* Description:
 *      Transition function to resume interception.
 *
 * Transition:
 *      ResumeIntercept -> ReadModFile | NewConnection | Send
 *
 * Steps:
 *      1. Receive the result from commander_response.receiver
 *
 *      2. If the result is Some, update the resume info, whether
 *         response needs to be intercepted (http req only)
 *
 *      3. If the file is modified, return ProxyState::ReadModFile.
 *
 *      4. Else if the file is not modified and resume_info.req_info is Some,
 *          return ProxyState::NewConnection (http req only)
 *
 *      5. Default, return ProxyState::Send
 *
 * Error:
 *      ProxyStateError::CommanderResponse      [1]
 *      ProxyStateError::WrongCommand           [2]
 */

pub async fn resume_intercept<T>(
    mut conn: T,
) -> Result<ProxyState<T>, ProxyStateError>
where
    T: UpdateResumeInfo + CanCommunicate,
{
    let response = conn
        .receiver()
        .recv()
        .await
        .ok_or(ProxyStateError::CommanderResponse("resume_intercept"))?;
    if response.is_drop_msg() {
        return Ok(ProxyState::Drop(conn));
    }
    // 2. If the result is Some, update the interception info, (http only)
    if let Some(resume_info) = Option::<ResumeInfo>::try_from(response)? {
        conn.update_resume_info(&resume_info);
        if resume_info.modified() {
            trace!("file mod| Y");
            return Ok(ProxyState::ReadModFile(conn, resume_info));
            // http req only
        } else if let Some(serve_info) = resume_info.into_server_info() {
            trace!("new conn");
            return Ok(ProxyState::NewConnection(conn, serve_info));
        };
    }
    trace!("file mod| N| => send");
    Ok(ProxyState::Send(conn))
}
