// src/handlers/todo.rs
use crate::config::Config;
use crate::errors::ApiError;
use crate::models::auth::Claims;
use crate::models::todo::{CreateTodoRequest, TodoResponse, UpdateTodoRequest};
use crate::solana::SolanaService;
use crate::utils::wallet::get_keypair_from_signature;
use actix_web::{web, HttpResponse};
use validator::Validate;

pub async fn create_todo(
    req: web::Json<CreateTodoRequest>,
    config: web::Data<Config>,
    claims: web::ReqData<Claims>,
) -> Result<HttpResponse, ApiError> {
    // Validate the request
    req.validate().map_err(|e| ApiError::BadRequest(e.to_string()))?;
    
    // Get the keypair from the JWT claims
    let keypair = get_keypair_from_signature(&claims.sub)?;
    
    // Create a Solana service instance
    let solana_service = SolanaService::new(&config)?;
    
    // Call the Solana program
    let todo = solana_service.create_todo(&keypair, req.into_inner())?;
    
    // Return the created todo
    Ok(HttpResponse::Created().json(todo))
}

pub async fn get_todos(
    config: web::Data<Config>,
    claims: web::ReqData<Claims>,
) -> Result<HttpResponse, ApiError> {
    // Create a Solana service instance
    let solana_service = SolanaService::new(&config)?;
    
    // Get todos for the wallet
    let todos = solana_service.get_todos_for_wallet(&claims.sub)?;
    
    // Return the todos
    Ok(HttpResponse::Ok().json(todos))
}

pub async fn update_todo(
    req: web::Json<UpdateTodoRequest>,
    path: web::Path<u64>,
    config: web::Data<Config>,
    claims: web::ReqData<Claims>,
) -> Result<HttpResponse, ApiError> {
    let task_id = path.into_inner();
    
    // Validate the request
    req.validate().map_err(|e| ApiError::BadRequest(e.to_string()))?;
    
    // Get the keypair from the JWT claims
    let keypair = get_keypair_from_signature(&claims.sub)?;
    
    // Create a Solana service instance
    let solana_service = SolanaService::new(&config)?;
    
    // Call the Solana program
    let todo = solana_service.update_todo(&keypair, task_id, req.into_inner())?;
    
    // Return the updated todo
    Ok(HttpResponse::Ok().json(todo))
}

pub async fn delete_todo(
    path: web::Path<u64>,
    config: web::Data<Config>,
    claims: web::ReqData<Claims>,
) -> Result<HttpResponse, ApiError> {
    let task_id = path.into_inner();
    
    // Get the keypair from the JWT claims
    let keypair = get_keypair_from_signature(&claims.sub)?;
    
    // Create a Solana service instance
    let solana_service = SolanaService::new(&config)?;
    
    // Call the Solana program
    solana_service.delete_todo(&keypair, task_id)?;
    
    // Return a success response
    Ok(HttpResponse::NoContent().finish())
}