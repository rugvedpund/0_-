use tokio::net::TcpStream;

use crate::proxy::states::ZStream;
use crate::repeater::conn::RepeaterConn;

// Convert RepeaterConn<ZStream> -> RepeaterConn<TcpStream>
impl From<(RepeaterConn<ZStream>, TcpStream)> for RepeaterConn<TcpStream> {
    fn from((conn, stream): (RepeaterConn<ZStream>, TcpStream)) -> Self {
        RepeaterConn {
            path: conn.path,
            stream,
            server_info: conn.server_info,
            update: conn.update,
        }
    }
}
