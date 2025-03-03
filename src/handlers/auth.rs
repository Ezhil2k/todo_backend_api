// src/handlers/auth.rs
use crate::config::Config;
use crate::errors::ApiError;
use crate::models::auth::{Claims, TokenResponse, WalletAuth};
use crate::utils::wallet::verify_wallet_signature;
use actix_web::{web, HttpResponse};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn authenticate(
    req: web::Json<WalletAuth>,
    config: web::Data<Config>,
) -> Result<HttpResponse, ApiError> {
    // Verify the wallet signature
    let is_valid = verify_wallet_signature(
        &req.public_key,
        &req.signature,
        &req.message,
    )?;
    
    if !is_valid {
        return Err(ApiError::Unauthorized("Invalid signature".to_string()));
    }
    
    // Calculate expiration (24 hours from now)
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| ApiError::InternalServerError("Clock error".to_string()))?
        .as_secs() as usize;
    
    let exp = now + 86400; // 24 hours
    
    // Create JWT claims
    let claims = Claims {
        sub: req.public_key.clone(),
        exp,
    };
    
    // Generate JWT token
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )?;
    
    // Return the token
    Ok(HttpResponse::Ok().json(TokenResponse {
        token,
        token_type: "Bearer".to_string(),
    }))
}