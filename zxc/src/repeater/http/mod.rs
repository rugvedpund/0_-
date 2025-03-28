pub mod roneonestruct;
use roneonestruct::*;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::states::rstate::RepeaterState;
use crate::async_step::async_run;
use crate::io::file::{FileErrorInfo, FileEvent};
use crate::proxy::handler_state::error::ProxyStateError;
use crate::repeater::conn::RepeaterConn;
use crate::repeater::http::Roneone;

pub async fn handle_http<T>(
    conn: RepeaterConn<T>,
) -> Result<Roneone<T>, ProxyStateError>
where
    T: AsyncWriteExt + AsyncReadExt + Unpin,
{
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&conn.path)
        .await
        .map_err(|e| FileErrorInfo::from((&conn.path, FileEvent::Open, e)))?;
    let one: Roneone<T> = Roneone::from((conn, file));
    let state = RepeaterState::ReadFromFile(one);
    match async_run(state).await? {
        RepeaterState::End(hconn) => Ok(hconn),
        _ => {
            unreachable!();
        }
    }
}
