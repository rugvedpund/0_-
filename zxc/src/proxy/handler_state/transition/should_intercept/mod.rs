use tracing::trace;

use crate::proxy::handler_state::{ProxyState, ProxyStateError};
mod query_commander_should_intercept;
mod trait_should_intercept;
pub use query_commander_should_intercept::*;
pub use trait_should_intercept::*;

/* Description:
 *      Transition function to check if http/ws request/response should be intercepted.
 *
 * Transition:
 *      ShouldIntercept -> Intercept | Send
 *
 * Steps:
 *      If should_intercept() returns
 *          1. Some(true), return Intercept
 *          2. None (ws response only), call query_commander_should_intercept()
 *             and return Intercept if true
 *
 *      Default to Send
 *
 * Returns:
 *      Ok(ProxyState::Intercept | ProxyState::Send)
 *
 * Errors:
 *      ws response only
 *          ProxyStateError::CommanderRequest
 *          ProxyStateError::WsShouldIntercept
 */

pub async fn should_intercept<T>(
    mut conn: T,
) -> Result<ProxyState<T>, ProxyStateError>
where
    T: ShouldIntercept + QueryCommanderShouldIntercept,
{
    let result = match conn.should_intercept() {
        Some(true) => {
            trace!("Y");
            true
        }
        None => {
            trace!("wres");
            conn.query_commander_should_intercept()
                .await?
        }
        _ => false,
    };
    if result {
        trace!("Y");
        return Ok(ProxyState::Intercept(conn));
    }
    trace!("N");
    Ok(ProxyState::Send(conn))
}
