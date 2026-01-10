use bytes::Bytes;
use fastwebsockets::FragmentCollector;
use fastwebsockets::WebSocketError;
use fastwebsockets::handshake;
use http_body_util::Empty;
use hyper::Method;
use hyper::Request;
use hyper::Uri;
use hyper::header;
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioExecutor;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use tokio_rustls::rustls::pki_types::ServerName;

use crate::tls::tls_connector;

pub type FrameCollector = FragmentCollector<TokioIo<Upgraded>>;

pub async fn connect(uri: &Uri) -> Result<FrameCollector, WebSocketError> {
    let host = uri.host().expect("not found host");
    let port = uri.port_u16().unwrap_or(443);

    let stream = TcpStream::connect((host, port)).await?;

    let req = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .header(header::HOST, host)
        .header(header::UPGRADE, "websocket")
        .header(header::CONNECTION, "upgrade")
        .header(
            header::SEC_WEBSOCKET_KEY,
            fastwebsockets::handshake::generate_key(),
        )
        .header(header::SEC_WEBSOCKET_VERSION, 13)
        .body(Empty::<Bytes>::new())
        .expect("invalid req");

    let (ws, _) = if uri
        .scheme_str()
        .is_some_and(|schema| schema == "https" || schema == "wss")
    {
        let connector = tls_connector();

        let domain = ServerName::try_from(host.to_owned()).expect("invalid domain");
        let tls_stream = connector.connect(domain, stream).await?;

        handshake::client(&TokioExecutor::new(), req, tls_stream).await?
    } else {
        handshake::client(&TokioExecutor::new(), req, stream).await?
    };

    Ok(FragmentCollector::new(ws))
}
