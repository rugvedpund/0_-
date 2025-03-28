use super::*;
use crate::proxy::states::{
    ClientTlsStream, ConnectionState, ServerTlsStream, Tcp
};

/* OneOneStruct<T,Tcp,Request> => ConnectionState<T>
 *
 * Used in:
 *      ProxyState::NewConnection
 *      when,
 *          client      = tcp://
 *          server      = tcp://
 *          new server  = tls://
 */

impl<T> From<OneOneStruct<T, Tcp, Request>> for ConnectionState<T>
where
    T: AsyncReadExt + AsyncWriteExt + Unpin,
{
    fn from(oneone: OneOneStruct<T, Tcp, Request>) -> Self {
        let (conn, addinfo) = oneone.into();
        ConnectionState::EstablishTcpTls(conn, addinfo)
    }
}

/* OneOneStruct<ServerTlsStream<T>,ClientTlsStream<Tcp>,Request> =>
 *          ConnectionState<T>
 *
 * Used in:
 *      ProxyState::NewConnection
 *      when,
 *          client      = tls://
 *          server      = tls://
 *          new server  = tcp://
 */

impl<T> From<OneOneStruct<ServerTlsStream<T>, ClientTlsStream<Tcp>, Request>>
    for ConnectionState<T>
where
    T: AsyncReadExt + AsyncWriteExt + Unpin,
{
    fn from(
        oneone: OneOneStruct<
            ServerTlsStream<T>,
            ClientTlsStream<Tcp>,
            Request,
        >,
    ) -> Self {
        let (conn, addinfo) = oneone.into();
        ConnectionState::EstablishTlsTcp(conn, addinfo)
    }
}

/* OneOneStruct<ServerTlsStream<T>,Tcp,Request> => ConnectionState<T>
 *
 *  Blank implementation
 *
 *  Should not reach this state, can_reconnect() should succeed in reconnect
 *  state
 */

impl<T> From<OneOneStruct<ServerTlsStream<T>, Tcp, Request>>
    for ConnectionState<T>
where
    T: AsyncReadExt + AsyncWriteExt + Unpin,
{
    fn from(_: OneOneStruct<ServerTlsStream<T>, Tcp, Request>) -> Self {
        unreachable!();
    }
}

/* OneOneStruct<T,ClientTlsStream<Tcp>,Request> => ConnectionState<T>
 *
 *  Blank implementation
 *
 *  Should not reach this state, can_reconnect() should succeed in reconnect
 *  state
 */

impl<T> From<OneOneStruct<T, ClientTlsStream<Tcp>, Request>>
    for ConnectionState<T>
{
    fn from(_: OneOneStruct<T, ClientTlsStream<Tcp>, Request>) -> Self {
        unreachable!();
    }
}
