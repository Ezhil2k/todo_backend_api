// src/middleware/auth.rs
use crate::config::Config;
use crate::errors::ApiError;
use crate::models::auth::Claims;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use futures::future::{ready, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct AuthMiddleware {
    config: Rc<Config>,
}

impl AuthMiddleware {
    pub fn new(config: Config) -> Self {
        AuthMiddleware {
            config: Rc::new(config),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service,
            config: self.config.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    config: Rc<Config>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        let config = self.config.clone();
        let service = self.service.clone();

        Box::pin(async move {
            match auth_header {
                Some(header) => {
                    let auth_str = header.to_str().map_err(|_| {
                        ApiError::Unauthorized("Invalid authorization header".to_string())
                    })?;

                    if !auth_str.starts_with("Bearer ") {
                        return Err(ApiError::Unauthorized("Invalid token format".to_string()).into());
                    }

                    let token = auth_str[7..].trim();
                    
                    let token_data = decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
                        &Validation::default(),
                    )
                    .map_err(|_| ApiError::Unauthorized("Invalid token".to_string()))?;

                    req.extensions_mut().insert(token_data.claims);
                    service.call(req).await
                }
                None => Err(ApiError::Unauthorized("Missing authorization header".to_string()).into()),
            }
        })
    }
}