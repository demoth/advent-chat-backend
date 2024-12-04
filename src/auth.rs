use axum::{
    http::StatusCode,
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::{info, error, instrument};

use crate::models::{LoginRequest, RegisterRequest, User};
use crate::storage::GLOBAL_STORAGE;

const JWT_SECRET: &[u8] = b"your-secret-key"; // In production, use a secure, environment-based secret

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[instrument]
pub async fn register(
    Json(req): Json<RegisterRequest>
) -> Result<Json<User>, StatusCode> {
    info!("Received register request for username: {}", req.username);

    // Check if username already exists
    if GLOBAL_STORAGE.get_user_by_username(&req.username).is_some() {
        error!("Username already exists: {}", req.username);
        return Err(StatusCode::CONFLICT);
    }

    // Hash password
    let password_hash = hash(&req.password, DEFAULT_COST)
        .map_err(|err| {
            error!("Password hashing failed: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Create new user
    let user = User {
        id: Uuid::new_v4().to_string(),
        username: req.username.clone(),
        password_hash,
    };

    GLOBAL_STORAGE.add_user(user.clone());
    info!("User registered successfully: {}", req.username);
    Ok(Json(user))
}

#[instrument]
pub async fn login(
    Json(req): Json<LoginRequest>
) -> Result<Json<String>, StatusCode> {
    info!("Login attempt for username: {}", req.username);

    // Find user by username
    let user = GLOBAL_STORAGE.get_user_by_username(&req.username)
        .ok_or_else(|| {
            error!("User not found: {}", req.username);
            StatusCode::UNAUTHORIZED
        })?;

    // Verify password
    verify(&req.password, &user.password_hash)
        .map_err(|err| {
            error!("Password verification failed for user {}: {:?}", req.username, err);
            StatusCode::UNAUTHORIZED
        })?;

    // Generate JWT
    let claims = Claims {
        sub: user.id.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET)
    ).map_err(|err| {
        error!("Token generation failed: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    info!("Login successful for username: {}", req.username);
    Ok(Json(token))
}

// Middleware for token validation
#[instrument]
pub fn validate_token(token: &str) -> Result<String, StatusCode> {
    use jsonwebtoken::{decode, DecodingKey, Validation};

    info!("Validating token");

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default()
    ).map_err(|err| {
        error!("Token validation failed: {:?}", err);
        StatusCode::UNAUTHORIZED
    })?;

    info!("Token validated successfully");
    Ok(token_data.claims.sub)
}