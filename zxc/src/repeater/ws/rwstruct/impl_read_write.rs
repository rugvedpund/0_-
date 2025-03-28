use super::*;

impl<T> ReadWrite for RWebSocket<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    type Error = WsError;
    type State = RepeaterState<Self>;

    /*  Transition:
     *      Receive -> WriteResponse
     *
     *  Error:
     *      WsError::Read
     */

    async fn read(mut self) -> Result<RepeaterState<Self>, WsError> {
        if let Some(msg) = self.stream.next().await {
            self.frame = Some(msg?);
        }
        trace!("Y");
        Ok(RepeaterState::WriteResponse(self))
    }

    // Transition: WriteResponse -> Receive | End
    async fn write(mut self) -> Result<RepeaterState<Self>, WsError> {
        let frame = self.frame.take().unwrap();
        let is_close = frame.is_close();
        self.stream
            .send(frame)
            .await
            .map_err(|_| WsError::Write)?;
        trace!("Y");
        if !is_close {
            trace!("continue");
            return Ok(RepeaterState::Receive(self));
        }
        trace!("close frame");
        Ok(RepeaterState::End(self))
    }
}
