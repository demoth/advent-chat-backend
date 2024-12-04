use crate::models::{User, Chat, Message};
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Storage {
    users: Arc<DashMap<String, User>>,
    chats: Arc<DashMap<String, Chat>>,
    messages: Arc<DashMap<String, Vec<Message>>>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            users: Arc::new(DashMap::new()),
            chats: Arc::new(DashMap::new()),
            messages: Arc::new(DashMap::new()),
        }
    }

    // User methods
    pub fn add_user(&self, user: User) {
        self.users.insert(user.id.clone(), user);
    }

    pub fn get_user_by_username(&self, username: &str) -> Option<User> {
        self.users.iter()
            .find(|entry| entry.username == username)
            .map(|entry| entry.clone())
    }

    pub fn get_user(&self, user_id: &str) -> Option<User> {
        self.users.get(user_id).map(|r| r.clone())
    }

    // Chat methods
    pub fn create_chat(&self, chat: Chat) {
        self.chats.insert(chat.id.clone(), chat);
    }

    pub fn get_chat(&self, chat_id: &str) -> Option<Chat> {
        self.chats.get(chat_id).map(|r| r.clone())
    }

    pub fn get_user_chats(&self, user_id: &str) -> Vec<Chat> {
        self.chats.iter()
            .filter(|entry| entry.participants.contains(user_id))
            .map(|entry| entry.clone())
            .collect()
    }

    // Message methods
    pub fn add_message(&self, message: Message) {
        self.messages
            .entry(message.chat_id.clone())
            .or_insert_with(Vec::new)
            .push(message);
    }

    pub fn get_chat_messages(&self, chat_id: &str) -> Vec<Message> {
        self.messages
            .get(chat_id)
            .map(|messages| messages.clone())
            .unwrap_or_default()
    }
}

// Global storage instance
lazy_static::lazy_static! {
    pub static ref GLOBAL_STORAGE: Storage = Storage::new();
}
