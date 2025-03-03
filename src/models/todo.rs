// src/models/todo.rs
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTodoRequest {
    #[validate(range(min = 1))]
    pub task_id: u64,
    
    #[validate(length(max = 280))]
    pub description: String,
    
    pub due_date: i64,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateTodoRequest {
    #[validate(length(max = 280))]
    pub description: Option<String>,
    
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoResponse {
    pub task_id: u64,
    pub description: String,
    pub completed: bool,
    pub due_date: i64,
    pub owner: String,
}