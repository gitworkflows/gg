use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage};
use futures_util::{StreamExt, SinkExt};
use url::Url;
use log::{info, error};
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum ClientMessage {
    Connected,
    Disconnected,
    Text(String),
    Binary(Vec<u8>),
    Error(String),
}

pub struct WebSocketClient {
    url: Url,
    tx: mpsc::Sender<ClientMessage>,
    _rx_handle: tokio::task::JoinHandle<()>,
    _ws_tx: Option<futures_util::channel::mpsc::Sender<WsMessage>>, // To send messages to the WS
}

impl WebSocketClient {
    pub async fn new(url_str: &str, tx: mpsc::Sender<ClientMessage>) -> anyhow::Result<Self> {
        let url = Url::parse(url_str)?;
        info!("Connecting to WebSocket: {}", url);

        let (ws_stream, _) = connect_async(&url).await?;
        info!("WebSocket connection established.");

        let (mut write, mut read) = ws_stream.split();
        let (ws_tx, mut ws_rx) = futures_util::channel::mpsc::channel::<WsMessage>(100);

        // Spawn task to send messages from internal channel to WebSocket
        let write_handle = tokio::spawn(async move {
            while let Some(msg) = ws_rx.next().await {
                if let Err(e) = write.send(msg).await {
                    error!("Failed to send WebSocket message: {:?}", e);
                    break;
                }
            }
            info!("WebSocket write task finished.");
        });

        // Spawn task to receive messages from WebSocket and send to main app
        let read_tx = tx.clone();
        let read_handle = tokio::spawn(async move {
            read_tx.send(ClientMessage::Connected).await.ok();
            while let Some(message) = read.next().await {
                match message {
                    Ok(WsMessage::Text(text)) => {
                        if read_tx.send(ClientMessage::Text(text)).await.is_err() {
                            error!("Failed to send text message to app, receiver dropped.");
                            break;
                        }
                    }
                    Ok(WsMessage::Binary(bin)) => {
                        if read_tx.send(ClientMessage::Binary(bin)).await.is_err() {
                            error!("Failed to send binary message to app, receiver dropped.");
                            break;
                        }
                    }
                    Ok(WsMessage::Ping(payload)) => {
                        // Automatically handled by tokio-tungstenite, but can log
                        info!("Received WebSocket Ping: {:?}", payload);
                    }
                    Ok(WsMessage::Pong(payload)) => {
                        info!("Received WebSocket Pong: {:?}", payload);
                    }
                    Ok(WsMessage::Close(close_frame)) => {
                        info!("WebSocket connection closed: {:?}", close_frame);
                        read_tx.send(ClientMessage::Disconnected).await.ok();
                        break;
                    }
                    Ok(WsMessage::Frame(_)) => {
                        // Raw frame, usually not handled directly
                    }
                    Err(e) => {
                        error!("WebSocket error: {:?}", e);
                        if read_tx.send(ClientMessage::Error(e.to_string())).await.is_err() {
                            error!("Failed to send error message to app, receiver dropped.");
                        }
                        break;
                    }
                }
            }
            info!("WebSocket read task finished.");
        });

        Ok(Self {
            url,
            tx,
            _rx_handle: read_handle,
            _ws_tx: Some(ws_tx),
        })
    }

    /// Sends a text message over the WebSocket.
    pub async fn send_text(&self, text: String) -> anyhow::Result<()> {
        if let Some(ws_tx) = &self._ws_tx {
            ws_tx.clone().send(WsMessage::Text(text)).await?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("WebSocket client not connected or sender dropped."))
        }
    }

    /// Sends binary data over the WebSocket.
    pub async fn send_binary(&self, data: Vec<u8>) -> anyhow::Result<()> {
        if let Some(ws_tx) = &self._ws_tx {
            ws_tx.clone().send(WsMessage::Binary(data)).await?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("WebSocket client not connected or sender dropped."))
        }
    }

    /// Closes the WebSocket connection.
    pub async fn close(&mut self) -> anyhow::Result<()> {
        if let Some(mut ws_tx) = self._ws_tx.take() {
            ws_tx.send(WsMessage::Close(None)).await?;
            info!("WebSocket close message sent.");
            Ok(())
        } else {
            info!("WebSocket already closed or not connected.");
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{timeout, Duration};
    use tokio::sync::mpsc;
    use futures_util::StreamExt;
    use tokio_tungstenite::tungstenite::Message as WsMessage;
    use tokio_tungstenite::accept_async;
    use tokio::net::TcpListener;

    async fn run_echo_server(listener: TcpListener) {
        let (socket, _) = listener.accept().await.unwrap();
        let ws_stream = accept_async(socket).await.unwrap();
        let (mut write, mut read) = ws_stream.split();

        tokio::spawn(async move {
            while let Some(message) = read.next().await {
                match message {
                    Ok(WsMessage::Text(text)) => {
                        write.send(WsMessage::Text(format!("Echo: {}", text))).await.unwrap();
                    }
                    Ok(WsMessage::Binary(bin)) => {
                        write.send(WsMessage::Binary(bin)).await.unwrap();
                    }
                    Ok(WsMessage::Close(_)) => {
                        break;
                    }
                    _ => {}
                }
            }
        });
    }

    #[tokio::test]
    async fn test_websocket_client_connect_and_send_text() -> anyhow::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        tokio::spawn(run_echo_server(listener));

        let (tx, mut rx) = mpsc::channel(10);
        let client = WebSocketClient::new(&format!("ws://{}", addr), tx).await?;

        // Wait for connected message
        let msg = timeout(Duration::from_secs(1), rx.recv()).await?.unwrap();
        assert_eq!(msg, ClientMessage::Connected);

        // Send text message
        client.send_text("Hello, server!".to_string()).await?;

        // Wait for echo response
        let msg = timeout(Duration::from_secs(1), rx.recv()).await?.unwrap();
        assert_eq!(msg, ClientMessage::Text("Echo: Hello, server!".to_string()));

        client.close().await?;
        let msg = timeout(Duration::from_secs(1), rx.recv()).await?.unwrap();
        assert_eq!(msg, ClientMessage::Disconnected);

        Ok(())
    }

    #[tokio::test]
    async fn test_websocket_client_send_binary() -> anyhow::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        tokio::spawn(run_echo_server(listener));

        let (tx, mut rx) = mpsc::channel(10);
        let client = WebSocketClient::new(&format!("ws://{}", addr), tx).await?;

        let _ = timeout(Duration::from_secs(1), rx.recv()).await?; // Connected message

        // Send binary message
        let binary_data = vec![1, 2, 3, 4, 5];
        client.send_binary(binary_data.clone()).await?;

        // Wait for echo response
        let msg = timeout(Duration::from_secs(1), rx.recv()).await?.unwrap();
        assert_eq!(msg, ClientMessage::Binary(binary_data));

        client.close().await?;
        Ok(())
    }
}
