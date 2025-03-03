// src/config.rs
use serde::Deserialize;
use solana_sdk::commitment_config::CommitmentConfig;
use std::env;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub solana_rpc_url: String,
    pub program_id: String,
    pub commitment: CommitmentConfig,
    pub jwt_secret: String,
}

pub fn load_config() -> Config {
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");
    
    let solana_rpc_url = env::var("SOLANA_RPC_URL")
        .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
    
    let program_id = env::var("PROGRAM_ID")
        .unwrap_or_else(|_| "hS4TFJW9MdMsCS3c7QWfvjfjEJBnm1pc6wfVAiBnzar".to_string());
    
    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    Config {
        host,
        port,
        solana_rpc_url,
        program_id,
        commitment: CommitmentConfig::confirmed(),
        jwt_secret,
    }
}