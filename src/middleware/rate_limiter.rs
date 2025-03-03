
// src/middleware/rate_limiter.rs
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::future::{ready, Ready};
use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use crate::errors::ApiError;

// Simple in-memory rate limiter
pub struct RateLimiter {
    max_requests: usize,
    window_seconds: u64,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_seconds: u64) -> Self {
        Self {
            max_requests,
            window_seconds,
        }
    }
}

#[derive(Clone)]
struct RateLimiterState {
    requests: HashMap<String, Vec<Instant>>,
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RateLimiterMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimiterMiddleware {
            service,
            state: Rc::new(RefCell::new(RateLimiterState {
                requests: HashMap::new(),
            })),
            max_requests: self.max_requests,
            window_seconds: self.window_seconds,
        }))
    }
}

pub struct RateLimiterMiddleware<S> {
    service: S,
    state: Rc<RefCell<RateLimiterState>>,
    max_requests: usize,
    window_seconds: u64,
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let peer_ip = match req.connection_info().peer_addr() {
            Some(ip) => ip.to_string(),
            None => "unknown".to_string(),
        };

        let now = Instant::now();
        let window = Duration::from_secs(self.window_seconds);
        let mut state = self.state.borrow_mut();

        // Clean up old requests
        if let Some(requests) = state.requests.get_mut(&peer_ip) {
            requests.retain(|&time| now.duration_since(time) < window);
        }

        // Check if rate limit exceeded
        match state.requests.get_mut(&peer_ip) {
            Some(requests) => {
                if requests.len() >= self.max_requests {
                    let service_error = ApiError::RateLimitExceeded;
                    return Box::pin(async move { Err(service_error.into()) });
                }
                requests.push(now);
            }
            None => {
                state.requests.insert(peer_ip, vec![now]);
            }
        }

        let fut = self.service.call(req);
        Box::pin(async move { fut.await })
    }
}