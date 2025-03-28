pub mod handler_state;
pub mod server_info;
pub mod states;

use tokio::net::{TcpListener, TcpStream};
use tokio::select;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{Instrument, Level, debug, error, span, trace};

use crate::CommanderRequest;
use crate::async_step::async_run;
use crate::proxy::states::ConnectionState;

/* Steps:
 *      ----- loop -----
 *          ----- select -----
 *          a. Cancellation token
 *          b. Accept new connections
 *              1. Spawn a new task for each connection
 *              2. Create a new ConnectionState
 *              3. Call async_run to start state machine
 *              4. Upon completion, close the connection with commander
 *          c. Remove completed tasks from handle
 *
 *      If token is cancelled, abort all tasks and cancel the task_token to
 *      signal ui tasks to close
 */

pub async fn start_proxy(
    tx: Sender<CommanderRequest>,
    listener: TcpListener,
    token: CancellationToken,
    task_token: CancellationToken,
) -> std::io::Result<()> {
    debug!("[*] Proxy Started");
    let mut tasks = JoinSet::new();
    let mut id = 1;
    loop {
        let tx_clone = tx.clone();
        select! {
            _ = token.cancelled() => {
                debug!("[*] Proxy Stopped");
                break;
            }
            result = listener.accept() => {
                match result {
                    Ok((stream, _)) => {
                        tasks.spawn(async move {
                            let span = span!(Level::TRACE, "Prx", id);
                            let _ = span.enter();
                            // 3. Create a new ConnectionState
                            let state = ConnectionState::<TcpStream>::new(
                                id,
                                stream,
                                tx_clone.clone(),
                            );
                            // 4. Call async_run to start state machine
                            if let Err(e) = async_run(state).instrument(span).await {
                                if e.is_common_error() {
                                    trace!("{}", e)
                                } else {
                                    error!("{}", e)
                                }
                            }
                            let request = CommanderRequest::Close(id);
                            let _ = tx_clone.send(request).await;
                        });
                        id += 1;
                    }
                    Err(e) => {
                        error!("accept| {}", e);
                    }
                }
            }
            _ = tasks.join_next() => {}
        }
    }
    tasks.abort_all();
    task_token.cancel();
    Ok(())
}
