use tokio::fs::File;
use tracing::trace;

use crate::interceptor::message::from_ui::resume_info::ResumeInfo;
use crate::io::file::{FileErrorInfo, rewrite_file};
use crate::proxy::handler_state::ProxyState;
use crate::proxy::handler_state::error::ProxyStateError;

/* Description:
 *      Trait to rewrite log.
 */

pub trait Rewrite {
    fn file_and_data(&mut self) -> (&mut File, &[u8]);
}

/* Description:
 *      Transition function to rewrite http request/response.
 *      ws connections should not reach this state.
 *
 * Transition:
 *      Rewrite -> NewConnection | Send
 *
 * Steps:
 *      1. Get File and data
 *      2. call rewrite_file
 *      3. If resume_info has req_info, return ProxyState::NewConnection
 *      4. Default, return ProxyState::Send
 *
 * Errors:
 *      ProxyStateError::ReWriteFile     [3]
 */

pub async fn rewrite_log<T>(
    mut conn: T,
    resume_info: ResumeInfo,
) -> Result<ProxyState<T>, ProxyStateError>
where
    T: Rewrite,
{
    let (file, data) = conn.file_and_data();
    rewrite_file(file, data)
        .await
        .map_err(|(event, e)| FileErrorInfo::from((file, event, e)))?;
    if let Some(info) = resume_info.into_server_info() {
        trace!("new connection");
        return Ok(ProxyState::NewConnection(conn, info));
    }

    trace!("send");
    Ok(ProxyState::Send(conn))
}
