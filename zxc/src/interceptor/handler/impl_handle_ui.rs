use super::InterceptorHandler;
use crate::commander::codec::perform_codec_op;
use crate::interceptor::error::InterceptorError;
use crate::interceptor::message::from_ui::{InterUIOps, InterUImsg};
use crate::run::boundary::HandleUI;

/* Steps
 *      1. Serialize self.buf to InterUImsg
 *
 *      2. Match Operation
 *
 *      3. If Toggle, change intercept_state to !intercept_state and send to
 *      commander
 *
 *      4. If Resume | Forward | Drop , send to commander
 *
 *      5. If Close, return InterceptorError::UIclosed
 *
 *      6. If encode | decode , call perform_codec_op(), set result to Some
 *
 *      7. If result is Some, return Some((msg.id, result))
 *
 * Errors:
 *      InterceptorError
 *          MsgDeSerialize      [1]
 *          CommanderSend       [3] [4]
 *          Codec               [5]
 *          UIclosed            [6]
 */

impl HandleUI for InterceptorHandler {
    async fn handle_ui(
        &mut self,
    ) -> Result<Option<(usize, String)>, InterceptorError> {
        let msg = serde_json::from_slice::<InterUImsg>(&self.buf)
            .map_err(|e| InterceptorError::from((&self.buf, e)))?;
        let id = msg.id;
        let result = match msg.op() {
            InterUIOps::Toggle
            | InterUIOps::Resume(_)
            | InterUIOps::Forward(_)
            | InterUIOps::Drop(..) => {
                if matches!(msg.op(), InterUIOps::Toggle) {
                    self.intercept_state = !self.intercept_state;
                }
                self.to_commander
                    .send(msg.into())
                    .await?;
                None
            }

            InterUIOps::Close => return Err(InterceptorError::UIclosed),
            InterUIOps::Encode {
                codec,
                data,
            } => {
                let result = perform_codec_op(true, codec, data.as_bytes())?;
                Some(result)
            }
            InterUIOps::Decode {
                codec,
                data,
            } => {
                let result = perform_codec_op(false, codec, data.as_bytes())?;
                Some(result)
            }
        };
        if let Some(result) = result {
            return Ok(Some((id, result.to_string())));
        }
        Ok(None)
    }
}
