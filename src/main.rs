// src/main.rs
use actix_web::{web, App, HttpServer, middleware};
use dotenv::dotenv;
use std::env;
use todo_api::config;
use todo_api::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = config::load_config();
    let app_data = web::Data::new(config);

    log::info!("Starting server at http://{}:{}", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(todo_api::middleware::rate_limiter::RateLimiter::new(100, 60)) // 100 requests per minute
            .app_data(app_data.clone())
            .service(
                web::scope("/api")
                    .configure(routes::todo_routes::config)
                    .configure(routes::auth_routes::config)
            )
            .service(routes::swagger::swagger_ui())
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}