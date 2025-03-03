// src/routes/swagger.rs
use actix_web::web;
use actix_web_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use utoipa_swagger_ui::Config;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::todo::create_todo,
        crate::handlers::todo::get_todos,
        crate::handlers::todo::update_todo,
        crate::handlers::todo::delete_todo,
        crate::handlers::auth::authenticate,
    ),
    components(
        schemas(
            crate::models::todo::CreateTodoRequest,
            crate::models::todo::UpdateTodoRequest,
            crate::models::todo::TodoResponse,
            crate::models::auth::WalletAuth,
            crate::models::auth::TokenResponse,
            crate::errors::ErrorResponse,
        )
    ),
    tags(
        (name = "todos", description = "Todo management endpoints"),
        (name = "auth", description = "Authentication endpoints"),
    ),
    info(
        title = "Solana Todo API",
        version = "1.0.0",
        description = "API for interacting with a Solana todo program",
    )
)]
pub struct ApiDoc;

pub fn swagger_ui() -> SwaggerUi {
    let openapi = ApiDoc::openapi();
    SwaggerUi::new("/swagger-ui/{_:.*}")
        .url("/api-docs/openapi.json", openapi)
        .config(Config::default())
}