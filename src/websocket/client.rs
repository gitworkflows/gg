use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage};
use futures_util::{StreamExt, SinkExt};
use url::Url;
use tokio::sync::mpsc;

pub struct WebSocketClient {
    url: Url,
    sender: Option<mpsc::UnboundedSender<WsMessage>>,
    receiver: Option<mpsc::UnboundedReceiver<WsMessage>>,
}

impl WebSocketClient {
    pub fn new(url_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let url = Url::parse(url_str)?;
        Ok(WebSocketClient {
            url,
            sender: None,
            receiver: None,
        })
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Connecting to WebSocket: {}", self.url);
        let (ws_stream, _) = connect_async(&self.url).await?;
        println!("WebSocket connected.");

        let (mut write, mut read) = ws_stream.split();
        let (tx, rx) = mpsc::unbounded_channel();
        let (event_tx, mut event_rx) = mpsc::unbounded_channel();

        self.sender = Some(tx);
        self.receiver = Some(event_rx);

        // Spawn task to send messages
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(e) = write.send(msg).await {
                    eprintln!("WebSocket send error: {}", e);
                    break;
                }
            }
        });

        // Spawn task to receive messages
        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(m) => {
                        if let Err(e) = event_tx.send(m) {
                            eprintln!("WebSocket event channel send error: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("WebSocket receive error: {}", e);
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn send(&self, message: WsMessage) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(sender) = &self.sender {
            sender.send(message)?;
            Ok(())
        } else {
            Err("WebSocket not connected".into())
        }
    }

    pub async fn receive(&mut self) -> Option<WsMessage> {
        if let Some(receiver) = &mut self.receiver {
            receiver.recv().await
        } else {
            None
        }
    }
}
