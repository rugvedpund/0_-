use bytes::BytesMut;
use bytes_to_frame::BytesToFrame;
use should_rewrite::ShouldRewrite;
use tracing::trace;

use super::frame_to_payload::FrameToPayload;
use crate::interceptor::message::from_ui::resume_info::ResumeInfo;
use crate::proxy::handler_state::ProxyState;
use crate::proxy::handler_state::error::ProxyStateError;
pub mod bytes_to_frame;
pub mod error;
pub mod should_rewrite;

/* Description:
 *      Transition Function for update frame.
 *
 * Transition:
 *      ResumeIntercept -> ReWrite | Send
 *
 * Steps:
 *      1. Parse buf to frame
 *      2. Add frame
 *      3. Convert frame to bytes
 *      4. If should_rewrite, return ProxyState::ReWrite
 *      5. return ProxyState::Send
 *
 * Error:
 *      ProxyStateError::UpdateFrame    [1]
 */

pub fn update_frame_state<T>(
    mut conn: T,
    buf: BytesMut,
    resume_info: ResumeInfo,
) -> Result<ProxyState<T>, ProxyStateError>
where
    T: BytesToFrame + FrameToPayload + ShouldRewrite,
{
    let frame = conn.parse_frame(buf)?;
    conn.add_frame(frame);
    conn.frame_to_payload();
    if conn.should_rewrite(&resume_info) {
        trace!("rewrite| Y");
        return Ok(ProxyState::ReWrite(conn, resume_info));
    }
    trace!("=> send");
    Ok(ProxyState::Send(conn))
}
