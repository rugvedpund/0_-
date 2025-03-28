use oneone::Response;
use protocol_traits::Frame;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::trace;

use super::Roneone;
use crate::io::write::write_and_flush;
use crate::proxy::handler_state::handlers::oneonestruct::OneOneRWError;
use crate::proxy::handler_state::handlers::read_http;
use crate::proxy::handler_state::read_write::ReadWrite;
use crate::repeater::states::rstate::RepeaterState;

impl<T> ReadWrite for Roneone<T>
where
    T: AsyncReadExt + AsyncWriteExt + Unpin,
{
    type Error = OneOneRWError;
    type State = RepeaterState<Self>;

    /* Transition:
     *      Receive -> WriteResponse
     *
     * Error:
     *      OneOneRWError::Read
     */

    async fn read(mut self) -> Result<RepeaterState<Self>, OneOneRWError> {
        let frame =
            read_http::<T, Response>(&mut self.stream, &mut self.buf).await?;
        self.payload = Some(frame.into_data());
        trace!("Y");
        Ok(RepeaterState::WriteResponse(self))
    }

    /* Transition:
     *      WriteResponse -> Receive
     */

    async fn write(mut self) -> Result<RepeaterState<Self>, OneOneRWError> {
        write_and_flush(&mut self.stream, self.payload.as_ref().unwrap())
            .await
            .map_err(OneOneRWError::Write)?;
        trace!("Y");
        Ok(RepeaterState::Receive(self))
    }
}
