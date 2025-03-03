// src/solana/mod.rs
use crate::config::Config;
use crate::errors::ApiError;
use crate::models::todo::{CreateTodoRequest, TodoResponse, UpdateTodoRequest};
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        system_program,
    },
    Client, Cluster, Program,
};
use std::str::FromStr;
use std::sync::Arc;

pub struct SolanaService {
    program: Program,
    program_id: Pubkey,
}

impl SolanaService {
    pub fn new(config: &Config) -> Result<Self, ApiError> {
        let program_id = Pubkey::from_str(&config.program_id)
            .map_err(|_| ApiError::BadRequest("Invalid program ID".to_string()))?;

        // Use a dummy keypair just for reading data
        // For write operations, we'll use the user's keypair
        let payer = Arc::new(Keypair::new());
        
        let cluster = Cluster::Custom(
            config.solana_rpc_url.clone(),
            config.solana_rpc_url.clone(),
        );
        
        let client = Client::new_with_options(
            cluster,
            payer,
            CommitmentConfig::confirmed(),
        );
        
        let program = client.program(program_id);
        
        Ok(SolanaService {
            program,
            program_id,
        })
    }
    
    pub fn get_todos_for_wallet(&self, wallet_pubkey: &str) -> Result<Vec<TodoResponse>, ApiError> {
        let wallet = Pubkey::from_str(wallet_pubkey)
            .map_err(|_| ApiError::BadRequest("Invalid wallet address".to_string()))?;
            
        // We're using getProgramAccounts with memcmp filters
        // This is a simplified approach - in a production app, you might want to use an indexer
        let accounts = self.program.accounts(vec![
            // Filter by owner field
            anchor_client::solana_sdk::rpc_filter::RpcFilterType::Memcmp(
                anchor_client::solana_sdk::rpc_filter::Memcmp {
                    offset: 8 + 8 + 4 + 280 + 1 + 8, // Position of owner field
                    bytes: anchor_client::solana_sdk::rpc_filter::MemcmpEncodedBytes::Base58(
                        wallet.to_string()
                    ),
                    encoding: Some(
                        anchor_client::solana_sdk::rpc_filter::MemcmpEncoding::Base58
                    ),
                }
            ),
        ])?;
        
        let mut todos = Vec::new();
        
        for (_, account) in accounts {
            todos.push(TodoResponse {
                task_id: account.task_id,
                description: account.description,
                completed: account.completed,
                due_date: account.due_date,
                owner: account.owner.to_string(),
            });
        }
        
        Ok(todos)
    }
    
    pub fn create_todo(
        &self,
        keypair: &Keypair,
        request: CreateTodoRequest,
    ) -> Result<TodoResponse, ApiError> {
        let owner = keypair.pubkey();
        let task_id = request.task_id;
        
        // Find the PDA for the todo account
        let seeds = [
            b"todo",
            owner.as_ref(),
            &task_id.to_le_bytes(),
        ];
        
        let (todo_account, bump) = Pubkey::find_program_address(
            &seeds,
            &self.program_id,
        );
        
        // Build and send the transaction
        let signature = self.program
            .request()
            .accounts(todo_solana_program::accounts::CreateTodo {
                todo: todo_account,
                owner,
                system_program: system_program::ID,
            })
            .args(todo_solana_program::instruction::CreateTodo {
                task_id,
                description: request.description.clone(),
                due_date: request.due_date,
            })
            .signer(keypair)
            .send()?;
            
        log::info!("Created todo with signature: {}", signature);
        
        Ok(TodoResponse {
            task_id,
            description: request.description,
            completed: false,
            due_date: request.due_date,
            owner: owner.to_string(),
        })
    }
    
    pub fn update_todo(
        &self,
        keypair: &Keypair,
        task_id: u64,
        request: UpdateTodoRequest,
    ) -> Result<TodoResponse, ApiError> {
        let owner = keypair.pubkey();
        
        // Find the PDA for the todo account
        let seeds = [
            b"todo",
            owner.as_ref(),
            &task_id.to_le_bytes(),
        ];
        
        let (todo_account, _) = Pubkey::find_program_address(
            &seeds,
            &self.program_id,
        );
        
        // First, fetch the current state to verify the account exists
        let todo = self.program.account::<todo_solana_program::TodoAccount>(todo_account)
            .map_err(|_| ApiError::NotFound(format!("Todo with ID {} not found", task_id)))?;
            
        // Handle description update if provided
        if let Some(description) = &request.description {
            let signature = self.program
                .request()
                .accounts(todo_solana_program::accounts::UpdateTodo {
                    todo: todo_account,
                    owner,
                })
                .args(todo_solana_program::instruction::UpdateDescription {
                    description: description.clone(),
                })
                .signer(keypair)
                .send()?;
                
            log::info!("Updated todo description with signature: {}", signature);
        }
        
        // Handle completed status update if provided
        if let Some(completed) = request.completed {
            if completed != todo.completed {
                let signature = self.program
                    .request()
                    .accounts(todo_solana_program::accounts::UpdateTodo {
                        todo: todo_account,
                        owner,
                    })
                    .args(todo_solana_program::instruction::ToggleCompleted {})
                    .signer(keypair)
                    .send()?;
                    
                log::info!("Toggled todo completion status with signature: {}", signature);
            }
        }
        
        // Fetch the updated state
        let updated_todo = self.program.account::<todo_solana_program::TodoAccount>(todo_account)?;
        
        Ok(TodoResponse {
            task_id: updated_todo.task_id,
            description: updated_todo.description,
            completed: updated_todo.completed,
            due_date: updated_todo.due_date,
            owner: updated_todo.owner.to_string(),
        })
    }
    
    pub fn delete_todo(
        &self,
        keypair: &Keypair,
        task_id: u64,
    ) -> Result<(), ApiError> {
        let owner = keypair.pubkey();
        
        // Find the PDA for the todo account
        let seeds = [
            b"todo", 
            owner.as_ref(),
            &task_id.to_le_bytes(),
        ];
        
        let (todo_account, _) = Pubkey::find_program_address(
            &seeds,
            &self.program_id,
        );
        
        // Check if the account exists
        let _ = self.program.account::<todo_solana_program::TodoAccount>(todo_account)
            .map_err(|_| ApiError::NotFound(format!("Todo with ID {} not found", task_id)))?;
            
        // Delete the todo
        let signature = self.program
            .request()
            .accounts(todo_solana_program::accounts::DeleteTodo {
                todo: todo_account,
                owner,
            })
            .args(todo_solana_program::instruction::DeleteTodo {})
            .signer(keypair)
            .send()?;
            
        log::info!("Deleted todo with signature: {}", signature);
        
        Ok(())
    }
}

// Define the account structure to match the Anchor program
#[derive(Debug, Clone)]
pub struct TodoAccount {
    pub task_id: u64,
    pub description: String,
    pub completed: bool,
    pub due_date: i64,
    pub owner: Pubkey,
}

pub mod todo_solana_program {
    use anchor_lang::prelude::*;
    
    #[derive(Debug, Clone)]
    pub struct TodoAccount {
        pub task_id: u64,
        pub description: String,
        pub completed: bool,
        pub due_date: i64, 
        pub owner: Pubkey,
    }
    
    // These modules are needed to mimic the structure of your Solana program for type-checking
    pub mod accounts {
        use anchor_lang::prelude::*;
        
        #[derive(Accounts)]
        pub struct CreateTodo<'info> {
            pub todo: AccountInfo<'info>,
            pub owner: AccountInfo<'info>,
            pub system_program: AccountInfo<'info>,
        }
        
        #[derive(Accounts)]
        pub struct UpdateTodo<'info> {
            pub todo: AccountInfo<'info>,
            pub owner: AccountInfo<'info>,
        }
        
        #[derive(Accounts)]
        pub struct DeleteTodo<'info> {
            pub todo: AccountInfo<'info>,
            pub owner: AccountInfo<'info>,
        }
    }
    
    pub mod instruction {
        pub struct CreateTodo {
            pub task_id: u64,
            pub description: String,
            pub due_date: i64,
        }
        
        pub struct UpdateDescription {
            pub description: String,
        }
        
        pub struct ToggleCompleted {}
        
        pub struct DeleteTodo {}
    }
}   