// src/routes/auth_routes.rs
use crate::handlers::auth;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(auth::authenticate))
    );
}
