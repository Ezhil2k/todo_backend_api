// src/routes/todo_routes.rs
use crate::handlers::todo;
use crate::middleware::auth::AuthMiddleware;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = AuthMiddleware::new(crate::config::load_config());
    
    cfg.service(
        web::scope("/todos")
            .wrap(auth)
            .route("", web::post().to(todo::create_todo))
            .route("", web::get().to(todo::get_todos))
            .route("/{id}", web::put().to(todo::update_todo))
            .route("/{id}", web::delete().to(todo::delete_todo))
    );
}
