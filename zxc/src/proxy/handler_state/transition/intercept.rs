use crate::commander::CommanderRequest;
use crate::id::Id;
use crate::interceptor::message::to_ui::InterToUI;
use crate::proxy::handler_state::transition::can_communicate::CanCommunicate;
use crate::proxy::handler_state::{ProxyState, ProxyStateError};

// Trait to intercept the http/ws request/response.
pub trait Intercept {
    fn get_inter_info(&self) -> InterToUI;
}

/* Description:
 *      Transition function to intercept the http/ws request/response.
 *
 * Transition:
 *      Intercept -> ResumeIntercept
 *
 * Steps:
 *      1. Get InterToUI, by calling get_inter_info() trait method.
 *      2. Wrap the msg in CommanderRequest::Intercept
 *      3. Send to commander.
 *
 * Error:
 *      ProxyStateError::Send  [4]
 */

pub async fn intercept<T>(
    mut conn: T,
) -> Result<ProxyState<T>, ProxyStateError>
where
    T: Intercept + CanCommunicate + Id,
{
    let info = conn.get_inter_info();
    let req = CommanderRequest::Intercept(conn.id(), info);
    conn.sender().send(req).await?;
    Ok(ProxyState::ResumeIntercept(conn))
}
