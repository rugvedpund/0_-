use super::WsStruct;
use crate::CommanderRequest;
use crate::commander::CommanderResponse;
use crate::proxy::handler_state::role::Role;
use crate::proxy::handler_state::{
    ProxyStateError, QueryCommanderShouldIntercept, ShouldIntercept
};

/* Description:
 *      Const function to return whether ws should be intercepted based on
 *      role.
 *
 * Steps:
 *      1. If role is server, return Some(true).
 *      2. Else, return None
 */

#[inline(always)]
const fn ws_should_intercept(role: &Role) -> Option<bool> {
    if let Role::Server = role {
        Some(true)
    } else {
        None
    }
}

impl<T, E> ShouldIntercept for WsStruct<T, E> {
    #[inline(always)]
    fn should_intercept(&self) -> Option<bool> {
        ws_should_intercept(&self.role)
    }
}

/* Steps:
 *      1. If role is client, build Communicate::ShouldInterceptWsRespone
 *         with id.
 *      2. Send the request to the commander.
 *      3. Receive the response from the commander.
 *      4. If the response is CommanderResponse::WsInterceptReply(true), return
 *         true.
 *
 * Errors:
 *      ProxyStateError::CommanderRequest   [2]
 *      ProxyStateError::WsShouldIntercept  [3]
 */

impl<T, E> QueryCommanderShouldIntercept for WsStruct<T, E> {
    async fn query_commander_should_intercept(
        &mut self,
    ) -> Result<bool, ProxyStateError> {
        if let Role::Client = self.role {
            let req = CommanderRequest::ShouldInterceptWsRespone(self.id);
            self.commander_sendr.send(req).await?;
            let result = self
                .commander_recvr
                .recv()
                .await
                .ok_or(ProxyStateError::WsShouldIntercept)?;
            Ok(matches!(result, CommanderResponse::WsInterceptReply(true)))
        } else {
            Ok(false)
        }
    }
}
