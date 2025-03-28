use crate::proxy::handler_state::error::ProxyStateError;

/* Description:
 *      Trait to check with commander whether ws response should be intercepted.
 *
 *      Ws implementation only.
 *
 * Error:
 *      ProxyStateError::CommanderRequest
 *      ProxyStateError::WsShouldIntercept
 */

pub trait QueryCommanderShouldIntercept {
    async fn query_commander_should_intercept(
        &mut self,
    ) -> Result<bool, ProxyStateError>;
}
