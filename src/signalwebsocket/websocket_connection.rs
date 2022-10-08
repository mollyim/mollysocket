use futures_util::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use native_tls::TlsConnector;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async_tls_with_config, tungstenite, tungstenite::client::IntoClientRequest,
    Connector::NativeTls, MaybeTlsStream, WebSocketStream,
};

pub struct WebSocketConnection {
    pub ws_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    pub ws_write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::Message>,
}

impl WebSocketConnection {
    pub async fn new(connect_addr: &str, tls_connector: TlsConnector) -> WebSocketConnection {
        let mut request = url::Url::parse(&connect_addr)
            .expect("Failed to parse URL")
            .into_client_request()
            .unwrap();
        request
            .headers_mut()
            .insert("X-Signal-Agent", http::HeaderValue::from_static("\"OWA\""));

        let (ws_stream, _) =
            connect_async_tls_with_config(request, None, Some(NativeTls(tls_connector)))
                .await
                .expect("Failed to connect");

        println!("WebSocket handshake has been successfully completed");

        let (ws_write, ws_read) = ws_stream.split();
        WebSocketConnection { ws_write, ws_read }
    }
}
