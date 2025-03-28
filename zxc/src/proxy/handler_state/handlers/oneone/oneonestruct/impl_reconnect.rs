use oneone::Request;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::trace;

use super::OneOneStruct;
use crate::io::socket::establish_connection;
use crate::proxy::handler_state::error::ProxyStateError;
use crate::proxy::handler_state::transition::reconnect::Reconnect;
use crate::proxy::server_info::ServerInfo;
use crate::proxy::states::ClientTlsStream;
use crate::proxy::states::connection::encrypt::server_encrypt;

/* Description:
 *      Reconnect trait imeplementation for OneOneHandler<T , Tcp, Request>
 *
 * Trait in:
 *      reconnect
 */

impl<T> Reconnect for OneOneStruct<T, TcpStream, Request>
where
    T: AsyncReadExt + AsyncWriteExt + Unpin,
{
    /* Description:
     *      Can reconnect to server
     *
     * Steps:
     *      If the new server to connect
     *          is tls  => false
     *          else    => true
     *
     */

    fn can_reconnect(&self, server_info: &ServerInfo) -> bool {
        !server_info.is_tls()
    }

    /* Description:
     *      Reconnect to server
     *
     * Steps:
     *      1. Establish Tcp connection to server address
     *      2. Set writer
     */
    async fn reconnect(&mut self) -> Result<(), ProxyStateError> {
        self.writer = establish_connection(self.address()).await?;
        Ok(())
    }
}

/* Description:
 *      Reconnect trait implementation for OneOneHandler<T , Tls, Request>
 *
 * Trait in:
 *      reconnect
 */

impl<T> Reconnect for OneOneStruct<T, ClientTlsStream<TcpStream>, Request>
where
    T: AsyncReadExt + AsyncWriteExt + Unpin,
{
    /* Description:
     *      Can reconnect to server
     *
     * Steps:
     *      If the new server to connect
     *          is tls  => true
     *          else    => false
     */

    fn can_reconnect(&self, server_info: &ServerInfo) -> bool {
        server_info.is_tls()
    }

    /* Description:
     *      Reconnect to server
     *
     * Steps:
     *      1. Establish Tcp connection to server address
     *      2. Get sni.
     *      3. Encrypt Tcp by calling server_encrypt
     *      4. Set writer
     */
    async fn reconnect(&mut self) -> Result<(), ProxyStateError> {
        let tcp = establish_connection(self.address()).await?;
        trace!("Reconnected");
        let server_name = self.server_info.sni().clone();
        let tls = server_encrypt(
            self.id,
            &mut self.commander_sendr,
            &mut self.commander_recvr,
            server_name,
            tcp,
        )
        .await?;
        trace!("Encrypted");
        self.writer = tls;
        Ok(())
    }
}
