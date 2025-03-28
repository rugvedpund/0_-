use std::fmt::{Debug, Display, Formatter};
use std::marker::Unpin;

use connection::encrypt::server_encrypt;
use oneone::Request;
use protocol_traits::Frame;
use tokio::io::{AsyncReadExt, AsyncWriteExt, copy_bidirectional_with_sizes};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot;
use tokio_rustls::StartHandshake;
pub use tokio_rustls::client::TlsStream as ClientTlsStream;
pub use tokio_rustls::server::TlsStream as ServerTlsStream;
use tracing::trace;

use crate::async_step::AsyncStep;
use crate::commander::{CommanderResponse, Protocol};
use crate::io::socket::establish_connection;
use crate::io::write::write_and_flush;
use crate::proxy::server_info::ServerInfo;
use crate::proxy::server_info::address::get_address;
use crate::{CAPACITY_2MB, CommanderRequest};
pub mod connection;
pub use connection::{Connection, ZStream};
pub mod error;
use error::*;

use super::handler_state::ProxyState;
use super::handler_state::additional_handler_info::AdditionalHandlerInfo;
use super::handler_state::handlers::oneonestruct::OneOneStruct;
use super::handler_state::handlers::{handle_http, read_http};

pub const PROXY_ESTABLISHED: &[u8; 39] =
    b"HTTP/1.1 200 Connection established\r\n\r\n";

// type alias
pub type Tcp = TcpStream;

pub enum ConnectionState<T> {
    ReadInitialClientData(Connection<T, ZStream>),
    DetermineEncryption(Connection<T, ZStream>),
    DetermineServer(Connection<T, ZStream>, bool),
    EstablishServerConnection(Connection<T, ZStream>, ServerInfo),
    ShouldProxy(Connection<T, Tcp>, ServerInfo),
    Relay(Connection<T, Tcp>, ServerInfo),
    ClientHandShake(
        Connection<T, Tcp>,
        Receiver<CommanderResponse>,
        ServerInfo,
    ),
    EncryptServer(
        Connection<StartHandshake<T>, Tcp>,
        Receiver<CommanderResponse>,
        ServerInfo,
    ),
    CompleteHandshake(
        Connection<StartHandshake<T>, ClientTlsStream<Tcp>>,
        Receiver<CommanderResponse>,
        ServerInfo,
    ),
    HandleTls(
        Connection<ServerTlsStream<T>, ClientTlsStream<Tcp>>,
        Receiver<CommanderResponse>,
        ServerInfo,
        Protocol,
    ),
    HandleTcp(
        Connection<T, Tcp>,
        Receiver<CommanderResponse>,
        ServerInfo,
        Protocol,
    ),
    EstablishTlsTcp(
        Connection<ServerTlsStream<T>, ClientTlsStream<Tcp>>,
        AdditionalHandlerInfo,
    ),
    HandleTlsTcp(Connection<ServerTlsStream<T>, Tcp>, AdditionalHandlerInfo),
    EstablishTcpTls(Connection<T, Tcp>, AdditionalHandlerInfo),
    HandleTcpTls(Connection<T, ClientTlsStream<Tcp>>, AdditionalHandlerInfo),
    End,
}

impl<T> ConnectionState<T>
where
    T: AsyncReadExt + AsyncWriteExt + Unpin,
{
    pub fn new(
        id: usize,
        client: T,
        tx: Sender<CommanderRequest>,
    ) -> ConnectionState<T> {
        Self::ReadInitialClientData(Connection::<T, ZStream>::new(
            id, client, tx,
        ))
    }
}

impl<T> AsyncStep for ConnectionState<T>
where
    T: AsyncReadExt + AsyncWriteExt + Unpin + Sync + Send + 'static + Debug,
{
    type Error = StateError;

    async fn next(self) -> Result<Self, StateError> {
        match self {
            /* Transition:
             *      ReadInitialClientData -> DetermineEncryption
             *
             * Steps:
             *      1. Read initial http data from client by calling
             *         read_http() with args (&mut T, &mut buf) which returns
             *         http request.
             *      2. on success, set frame to Some(http_request)
             *
             * Errors:
             *      StateError::ReadingInitialData
             */
            Self::ReadInitialClientData(mut conn) => {
                let frame =
                    read_http::<T, Request>(&mut conn.reader, &mut conn.buf)
                        .await
                        .map_err(StateError::InitialRead)?;
                conn.frame = Some(frame);
                Ok(Self::DetermineEncryption(conn))
            }

            /* Transition:
             *      DetermineEncryption -> DetermineServer
             *
             * Steps:
             *      frame.is_connect_request() to determine tls
             */
            Self::DetermineEncryption(conn) => {
                // Safe to unwrap frame as it has been set in previous state
                let tls = conn
                    .frame
                    .as_ref()
                    .unwrap()
                    .is_connect_request();
                trace!(tls);
                Ok(Self::DetermineServer(conn, tls))
            }

            /* Transition:
             *      DetermineServer -> EstablishServerConnection
             *
             * Steps:
             *      call get_address() with request.infoline and tls to get
             *      server Address
             *
             * Errors:
             *      StateError::Address
             */
            Self::DetermineServer(mut conn, tls) => {
                // Safe to unwrap frame as it has been set in Read Initial
                // Client Data
                let addr = get_address(
                    conn.frame
                        .as_mut()
                        .unwrap()
                        .infoline_as_mut(),
                    tls,
                )?;
                let server_info = ServerInfo::new(addr, tls, None);
                trace!("{}", server_info);
                Ok(Self::EstablishServerConnection(conn, server_info))
            }

            /* Transition:
             *      EstablishServerConnection -> ShouldProxy
             *
             * Steps:
             *      1. call establish_connection() with conn.address()
             *      2. on success, build connection with tcp
             *      [ From trait implemented in connection/convert.rs ]
             *
             * Errors:
             *      StateError::ServerConnect
             */
            Self::EstablishServerConnection(conn, server_info) => {
                let stream =
                    establish_connection(server_info.address()).await?;
                trace!("Y");
                let conn = Connection::from((conn, stream));
                Ok(Self::ShouldProxy(conn, server_info))
            }

            /* Transition:
             *      ShouldProxy ->  Relay | ClientHandShake | SwitchTcp
             *
             * Steps:
             *      1. Build one_shot channel to query commander whether the
             *      connection should be intercepted or relayed.
             *      2. Build Communicate::ShouldProxy with channel and
             *      server addr as string
             *      3. Send Request to Commander and Receive Response
             *      Option<mpsc::Receiver<CommanderReponse>> from Commander
             *      4. If Some,
             *          - if tls    => ClientHandShake
             *          - else      => SwitchTcp
             *      5. Else,    => Relay
             *
             * Errors:
             *      StateError::CommanderSend   [3]
             *      StateError::ShouldProxy     [4]
             */
            Self::ShouldProxy(conn, server_info) => {
                let (tx, rx) = oneshot::channel();
                let request = CommanderRequest::ShouldProxy(
                    conn.id,
                    server_info.address().to_string(),
                    tx,
                );
                conn.commander.send(request).await?;

                match rx.await? {
                    Some(recvr) => {
                        trace!("Y");
                        if server_info.is_tls() {
                            Ok(Self::ClientHandShake(conn, recvr, server_info))
                        } else {
                            Ok(Self::HandleTcp(
                                conn,
                                recvr,
                                server_info,
                                Protocol::OneOne,
                            ))
                        }
                    }
                    _ => {
                        trace!("N");
                        Ok(Self::Relay(conn, server_info))
                    }
                }
            }

            /* Transition:
             *      Relay   ->  End
             *
             * Steps:
             *      1. If tls, write PROXY_ESTABLISHED to client to indicate
             *      that server conn has been established successfully.
             *      2. If http, convert http_frame to data and send it to server.
             *      3. copy_bidirectional_with_sizes() from client to server
             *
             * Errors:
             *      StateError::ClientWrite     [1]
             *      StateError::ServerWrite     [2]
             */
            Self::Relay(mut conn, server_info) => {
                if server_info.is_tls() {
                    write_and_flush(&mut conn.reader, PROXY_ESTABLISHED)
                        .await
                        .map_err(StateError::ClientWrite)
                } else {
                    let data = conn.frame.take().unwrap().into_data();
                    write_and_flush(&mut conn.writer, &data)
                        .await
                        .map_err(StateError::ServerWrite)
                }?;
                // copy_bidirectional
                let _ = copy_bidirectional_with_sizes(
                    &mut conn.reader,
                    &mut conn.writer,
                    CAPACITY_2MB,
                    CAPACITY_2MB,
                )
                .await;
                trace!("Y");
                Ok(Self::End)
            }

            /* Transition:
             *      ClientHandShake -> EncryptServer
             *
             * Steps:
             *      1. write PROXY_ESTABLISHED to client to indicate that
             *         server conn has been established and client can send
             *         client_hello.
             *      2. call perform_handshake() on conn to receive client_hello
             *
             * Errors:
             *      StateError::ClientWrite         [1]
             *      StateError::ClientHandshake     [2]
             */
            Self::ClientHandShake(mut conn, recvr, server_info) => {
                write_and_flush(&mut conn.reader, PROXY_ESTABLISHED)
                    .await
                    .map_err(StateError::ClientWrite)?;
                let conn = conn.perform_handshake().await?;
                trace!("Y");
                Ok(Self::EncryptServer(conn, recvr, server_info))
            }

            /* Transition:
             *      EncryptServer -> CompleteHandshake
             *
             * Errors:
             *      StateError::InvalidDns
             *      StateError::ServerEncrypt
             */
            Self::EncryptServer(conn, mut recvr, mut server_info) => {
                let conn = conn
                    .encrypt_server(&mut recvr, &mut server_info)
                    .await?;
                trace!("Y");
                Ok(Self::CompleteHandshake(conn, recvr, server_info))
            }

            /* Transition:
             *      CompleteHandshake -> SwitchTls
             *
             * Errors:
             *      StateError::ClientEncrypt
             */
            Self::CompleteHandshake(conn, mut recvr, server_info) => {
                let conn = conn
                    .complete_handshake(&mut recvr, &server_info)
                    .await?;
                trace!("Y");
                Ok(Self::HandleTls(conn, recvr, server_info, Protocol::OneOne))
            }

            /* Description:
             *      Handles tls to tls
             *
             * Transition:
             *      HandleTls ->  EstablishTlsTcp | End
             *
             * Steps:
             *      1. Build one_one_request handler from conn
             *      [ From trait in oneonestruct/convert/from_connection ]
             *      2. Take the frame, i.e. CONNECT request
             *      3. Build ProxyState::Receive
             *      4. Call handle_http with ProxyState
             */
            Self::HandleTls(conn, recvr, server_info, _protocol) => {
                let mut client = OneOneStruct::<_, _, Request>::from((
                    conn,
                    recvr,
                    server_info,
                ));
                client.frame.take();
                let client_state = ProxyState::Receive(client);
                handle_http(client_state).await
            }

            /* Description:
             *      Handles tcp to tcp
             *
             * Transition:
             *      HandleTcp ->  EstablishTcpTls | End
             *
             * Steps:
             *      1. build one_one_request handler from conn
             *      [ From trait in oneonestruct/convert/from_connection ]
             *      2. build ProxyState::ShouldLog, Since request is already
             *      received
             *      3. call handle_http with ProxyState
             */
            Self::HandleTcp(conn, recvr, server_info, _protocol) => {
                let client = OneOneStruct::<_, _, Request>::from((
                    conn,
                    recvr,
                    server_info,
                ));
                let client_state = ProxyState::ShouldLog(client);
                handle_http(client_state).await
            }

            /* Description:
             *      Establish new tcp server connection from original tls-tls
             *      connection.
             *
             * Transition:
             *      EstablishTlsTcp -> HandleTlsTcp
             *
             * Steps:
             *      1. Establish the connection to conn.address()
             *      2. Build connection with tcp
             *      [From trait in connection/convert.rs ]
             *
             * Errors:
             *      StateError::ServerConnect
             */
            Self::EstablishTlsTcp(conn, addinfo) => {
                let tcp = establish_connection(addinfo.address()).await?;
                let conn = Connection::from((conn, tcp));
                trace!("Y");
                Ok(Self::HandleTlsTcp(conn, addinfo))
            }

            /* Description:
             *      Handles tls to tcp
             *
             * Transition:
             *      HandleTlsTcp ->  End
             *
             * Steps:
             *      1. Build one_one_request handler from conn and
             *      AdditionalHandlerInfo [ From trait implementation in
             *      oneonestruct/convert/from_connection_add_info ]
             *
             *      2. build ProxyState::Send
             *
             *      3. call handle_http with ProxyState
             */
            Self::HandleTlsTcp(conn, addinfo) => {
                let oneone =
                    OneOneStruct::<_, _, Request>::from((conn, addinfo));
                let client_state = ProxyState::Send(oneone);
                handle_http(client_state).await
            }

            /* Description:
             *      Establish new tls server connection from original tcp-tcp
             *
             * Transition:
             *      EstablishTcpTls -> HandleTcpTls
             *
             * Steps:
             *      1. Establish tcp connection to conn.address()
             *      2. call server_encrypt() to encrypt the connection
             *      3. Build connection with tls
             *      [From trait in connection/convert.rs ]
             *
             * Errors:
             *      StateError::ServerConnect
             *      StateError::ServerEncrypt
             */
            Self::EstablishTcpTls(mut conn, mut addinfo) => {
                let tcp = establish_connection(addinfo.address()).await?;
                let sni = addinfo.sni().to_owned();
                let tls = server_encrypt(
                    conn.id,
                    &mut conn.commander,
                    &mut addinfo.receiver,
                    sni,
                    tcp,
                )
                .await?;
                let conn = Connection::from((conn, tls));
                trace!("Y");
                Ok(Self::HandleTcpTls(conn, addinfo))
            }

            /* Description:
             *      Handles tcp to tls
             *
             * Transition:
             *      HandleTcpTls ->  End
             *
             * Steps:
             *      1. build one_one_request handler from conn and
             *      AdditionalHandlerInfo [ From trait implementation in
             *      oneonestruct/convert/from_connection_add_info ]
             *      2. build ProxyState::Send
             *      3. call handle_http with ProxyState
             */
            Self::HandleTcpTls(conn, addinfo) => {
                let oneone =
                    OneOneStruct::<_, _, Request>::from((conn, addinfo));
                let client_state = ProxyState::Send(oneone);
                handle_http(client_state).await
            }
            Self::End => Ok(Self::End),
        }
    }

    // Method to check if the state machine has ended
    fn is_ended(&self) -> bool {
        matches!(self, Self::End)
    }
}

impl<T> Display for ConnectionState<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::ReadInitialClientData(_) => "initial_read",
            Self::DetermineEncryption(_) => "encryption",
            Self::DetermineServer(..) => "server",
            Self::EstablishServerConnection(..) => "server conn",
            Self::ShouldProxy(..) => "should_proxy",
            Self::Relay(..) => "relay",
            Self::ClientHandShake(..) => "client_handshake",
            Self::EncryptServer(..) => "server_encrypt",
            Self::CompleteHandshake(..) => "client_encrypt",
            Self::HandleTls(_, _, info, _)
            | Self::HandleTcp(_, _, info, _) => &info.to_string(),
            Self::HandleTlsTcp(_, addinfo)
            | Self::HandleTcpTls(_, addinfo) => {
                &addinfo.server_info.to_string()
            }
            Self::EstablishTlsTcp(_, addinfo) => {
                &format!("establish_tls_tcp| {}", addinfo.server_info)
            }
            Self::EstablishTcpTls(_, addinfo) => {
                &format!("establish_tcp_tls| {}", addinfo.server_info)
            }
            Self::End => "",
        };
        write!(f, "{}", s)
    }
}
