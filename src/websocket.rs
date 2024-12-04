use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query,
    },
    response::{Response, IntoResponse},
    http::StatusCode,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::auth;
use crate::models::{Message as ChatMessage, Chat};
use crate::storage::GLOBAL_STORAGE;

// Shared state for WebSocket connections
#[derive(Default)]
struct SocketState {
    users: HashMap<String, Vec<String>>, // user_id -> [socket_ids]
}

lazy_static::lazy_static! {
    static ref SOCKET_CONNECTIONS: Mutex<SocketState> = Mutex::new(SocketState::default());
}

#[derive(Deserialize)]
pub struct ConnectParams {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub enum WebSocketMessage {
    CreateChat { name: String, participants: Vec<String> },
    SendMessage { chat_id: String, content: String },
    JoinChat { chat_id: String },
}

pub async fn handler(
    ws: WebSocketUpgrade,
    Query(params): Query<ConnectParams>,
) -> impl IntoResponse {
    // Validate token
    let user_id = match auth::validate_token(&params.token) {
        Ok(id) => id,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    ws.on_upgrade(move |socket| handle_socket(socket, user_id))
}

async fn handle_socket(socket: WebSocket, user_id: String) {
    let (mut sender, mut receiver) = socket.split();
    let socket_id = Uuid::new_v4().to_string();

    // Register socket connection
    {
        let mut state = SOCKET_CONNECTIONS.lock().await;
        state.users.entry(user_id.clone())
            .or_insert_with(Vec::new)
            .push(socket_id.clone());
    }

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            match serde_json::from_str::<WebSocketMessage>(&text) {
                Ok(WebSocketMessage::CreateChat { name, participants }) => {
                    let chat = Chat {
                        id: Uuid::new_v4().to_string(),
                        name,
                        is_group: participants.len() > 2,
                        participants: participants.into_iter().collect(),
                    };
                    GLOBAL_STORAGE.create_chat(chat);
                }
                Ok(WebSocketMessage::SendMessage { chat_id, content }) => {
                    let message = ChatMessage {
                        id: Uuid::new_v4().to_string(),
                        chat_id: chat_id.clone(),
                        sender_id: user_id.clone(),
                        content,
                        timestamp: chrono::Utc::now().timestamp() as u64,
                    };
                    GLOBAL_STORAGE.add_message(message);
                }
                Ok(WebSocketMessage::JoinChat { chat_id }) => {
                    if let Some(mut chat) = GLOBAL_STORAGE.get_chat(&chat_id) {
                        chat.participants.insert(user_id.clone());
                        GLOBAL_STORAGE.create_chat(chat);
                    }
                }
                Err(_) => {
                    // Handle invalid message
                }
            }
        }
    }

    // Cleanup socket connection
    {
        let mut state = SOCKET_CONNECTIONS.lock().await;
        if let Some(sockets) = state.users.get_mut(&user_id) {
            sockets.retain(|id| id != &socket_id);
        }
    }
}
