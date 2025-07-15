use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tokio_tungstenite::tungstenite::Message as WsMessage;

use crate::websocket::client::WebSocketClient; // Updated import

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationEvent {
    pub session_id: Uuid,
    pub user_id: String,
    pub event_type: EventType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    CommandExecuted { command: String },
    BlockAdded { block_id: Uuid },
    CursorMoved { position: usize },
    UserJoined { username: String },
    UserLeft { username: String },
}

pub struct CollaborationManager {
    session_id: Uuid,
    user_id: String,
    connected_users: Vec<String>,
    websocket_client: Option<WebSocketClient>, // Use the new client
}

impl CollaborationManager {
    pub fn new() -> Self {
        CollaborationManager {
            session_id: Uuid::new_v4(),
            user_id: "anonymous".to_string(),
            connected_users: Vec::new(),
            websocket_client: None,
        }
    }

    pub async fn connect(&mut self, url: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = WebSocketClient::new(&url)?;
        client.connect().await?;
        self.websocket_client = Some(client);
        Ok(())
    }

    pub async fn send_event(&self, event: CollaborationEvent) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(client) = &self.websocket_client {
            let serialized = serde_json::to_string(&event)?;
            client.send(WsMessage::Text(serialized)).await?;
            Ok(())
        } else {
            Err("Not connected to collaboration server".into())
        }
    }

    pub async fn receive_event(&mut self) -> Option<CollaborationEvent> {
        if let Some(client) = &mut self.websocket_client {
            if let Some(msg) = client.receive().await {
                if let WsMessage::Text(text) = msg {
                    return serde_json::from_str(&text).ok();
                }
            }
        }
        None
    }

    pub fn create_command_event(&self, command: String) -> CollaborationEvent {
        CollaborationEvent {
            session_id: self.session_id,
            user_id: self.user_id.clone(),
            event_type: EventType::CommandExecuted { command },
            timestamp: Utc::now(),
        }
    }

    pub fn start_session(&self, _session_id: &str) {
        println!("Starting collaboration session: {}", _session_id);
    }

    pub fn join_session(&self, _session_id: &str) {
        println!("Joining collaboration session: {}", _session_id);
    }

    pub fn send_update(&self, _update: &str) {
        println!("Sending collaboration update: {}", _update);
    }
}
