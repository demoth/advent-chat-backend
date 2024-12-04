use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use crate::models::Chat;
use crate::storage::GLOBAL_STORAGE;

pub async fn create_chat(
    Json(chat): Json<Chat>
) -> Result<Json<Chat>, StatusCode> {
    GLOBAL_STORAGE.create_chat(chat.clone());
    Ok(Json(chat))
}

pub async fn get_user_chats(
    State(user_id): State<String>
) -> Result<Json<Vec<Chat>>, StatusCode> {
    let chats = GLOBAL_STORAGE.get_user_chats(&user_id);
    Ok(Json(chats))
}
