// src/errors.rs
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use solana_client::client_error::ClientError;
use std::convert::From;

#[derive(Debug, Display, PartialEq)]
pub enum ApiError {
    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),
    
    #[display(fmt = "Unauthorized: {}", _0)]
    Unauthorized(String),
    
    #[display(fmt = "Forbidden: {}", _0)]
    Forbidden(String),
    
    #[display(fmt = "Not Found: {}", _0)]
    NotFound(String),
    
    #[display(fmt = "Internal Server Error: {}", _0)]
    InternalServerError(String),
    
    #[display(fmt = "Solana Error: {}", _0)]
    SolanaError(String),
    
    #[display(fmt = "Rate Limit Exceeded")]
    RateLimitExceeded,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            status: status_code.to_string(),
            message: self.to_string(),
        };
        
        HttpResponse::build(status_code).json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::SolanaError(_) => StatusCode::BAD_GATEWAY,
            ApiError::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
        }
    }
}

impl From<ClientError> for ApiError {
    fn from(error: ClientError) -> Self {
        ApiError::SolanaError(error.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        ApiError::Unauthorized(error.to_string())
    }
}

impl From<std::io::Error> for ApiError {
    fn from(error: std::io::Error) -> Self {
        ApiError::InternalServerError(error.to_string())
    }
}