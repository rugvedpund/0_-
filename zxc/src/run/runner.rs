use std::time::Duration;

use tokio::net::{UnixListener, UnixStream};
use tokio::select;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, trace};

use super::boundary::*;
use crate::io::unix_sock::error::UnixSockError;
use crate::io::unix_sock::reader::read_from_unix;
use crate::io::unix_sock::writer::build_slice_and_write;
use crate::io::write::write_and_flush;

const CLOSE_MSG: &[u8] = br#"[0,{"Action": "Close"}]"#;

/* Steps:
 * ----- loop -----
 *      ----- select -----
 *          a. Accept new connections
 *              1. Try to write &[0], if err old connection closed, accept
 *              new connection
 *              2. set it to stream
 *              3. flush storage
 *
 *          b. Receive msg from commander, call handle_commander_ui and match
 *          result
 *              1. Ok(None)         => continue
 *
 *              2. Ok(Some(data))   => buil_slice_and_write(0, data) to stream
 *                                     0, default ui handler needs to be called
 *
 *              3. Err(..)          => if err.needs_flush() (history only),
 *                                     call flush_storage() on stream.
 *
 *                                     else, print error
 *
 *          c. Read from stream
 *              1. If Err is Block, continue for other types return
 *              2. Call handle_ui() to get resutl.
 *              3. Clear buffer
 *              4. match result
 *                  a. Ok(None)             => continue
 *
 *                  b. Ok(Some((id,data))   => buil_slice_and_write(id, data)
 *                                             to stream
 *
 *                  c. Err(..)              => If is_ui_error() return,
 *                                             else continue
 *
 *          d. Check token cancelled,
 *              1. write CLOSE_MSG to stream
 *              2. check if ui closed properly by writing &[0] to stream is err
 *              3. If not err, sleep 500 ms and recheck
 *
 *  Errors:
 *      UnixSocketError::Accept     [1.a]
 *      UnixSocketError::Write      [4]
 */

pub async fn run_module<T>(
    handler: &mut T,
    listener: &UnixListener,
    mut stream: UnixStream,
    token: &CancellationToken,
) -> Result<(), <T as HandleCommander>::Error>
where
    T: FromCommander + HandleCommander + Buffer + HandleUI + FlushStorage,
{
    info!("running");
    loop {
        select! {
            result = listener.accept() => {
                if stream.try_write(&[0]).is_err() {
                    stream = result.map_err(UnixSockError::Accept)?.0;
                    trace!("new connection");
                    handler.flush_storage(&mut stream).await?;
                } else {
                    debug!("socket already bound");
                }
            }
            result = handler.recv() => {
                trace!("from commander");
                if let Some(msg) = result {
                    match handler.handle_commander_ui(msg).await
                    {
                        Ok(None) => (),
                        Ok(Some(data)) => build_slice_and_write(0, data, &mut stream).await?,
                        Err(e) => {
                            if e.needs_flush() {
                                trace!("needs flush");
                                handler.flush_storage(&mut stream).await?;
                                continue;
                            }
                            error!("handle commander| {}", e);
                        }
                    }
                }
            }
            result = stream.readable() => {
                trace!("from socket");
                match read_from_unix(result, &mut stream, handler.buf_as_mut()) {
                    Ok(_) => (),
                    Err(e) => {
                        if matches!(e, UnixSockError::Block) {
                            continue
                        }
                        return Err(e.into());
                    }
                }
                let result = handler.handle_ui().await;
                handler.buf_as_mut().clear();
                trace!("buffer cleared");
                match result {
                    Ok(None) => (),
                    Ok(Some((id, data))) => {
                        build_slice_and_write(id, data, &mut stream).await?;
                    }
                    Err(e) => {
                        if e.is_ui_error() {
                            return Err(e);
                        }
                        error!("handle ui| {}", e);
                    }
                }
            }
            _ = token.cancelled() => {
                write_and_flush(&mut stream, CLOSE_MSG).await.map_err(UnixSockError::Write)?;
                loop {
                    if write_and_flush(&mut stream, &[0]).await.is_err() {
                        trace!("runner| cancelled");
                        return Ok(())
                    }
                    sleep(Duration::from_millis(10)).await;
                }
            }
        }
    }
}
