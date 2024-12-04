# Rust Chat Backend

## Features
- User registration and authentication
- WebSocket-based real-time messaging
- In-memory storage for users, chats, and messages
- Support for direct and group chats

## Running the Project
```bash
cargo run
```

## API Endpoints
- `POST /register`: Register a new user
- `POST /login`: Authenticate and receive JWT token
- `GET /ws`: WebSocket connection endpoint (requires JWT token)

## WebSocket Message Types
- `CreateChat`: Create a new chat
- `SendMessage`: Send a message to a specific chat
- `JoinChat`: Join an existing chat

## Authentication
Uses JWT (JSON Web Tokens) for authentication. Include the token in WebSocket connection parameters.

## Technologies
- Axum web framework
- Tokio async runtime
- WebSocket for real-time communication
- In-memory storage with DashMap
- JWT for authentication

## TODO
- Persistent storage
- More robust error handling
- Enhanced authentication
- Message history retrieval
