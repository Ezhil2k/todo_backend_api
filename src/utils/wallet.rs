// src/utils/wallet.rs
use crate::errors::ApiError;
use solana_sdk::{
    signature::{Keypair, Signature},
    signer::keypair::keypair_from_seed,
    pubkey::Pubkey,
};
use std::str::FromStr;
use ed25519_dalek::{PublicKey, Signature as Ed25519Signature, Verifier};
use base58::FromBase58;
use std::convert::TryFrom;

pub fn verify_wallet_signature(
    public_key: &str,
    signature: &str,
    message: &str,
) -> Result<bool, ApiError> {
    // Decode the public key
    let pubkey_bytes = Pubkey::from_str(public_key)
        .map_err(|_| ApiError::BadRequest("Invalid public key".to_string()))?
        .to_bytes();
    
    let public_key = PublicKey::from_bytes(&pubkey_bytes)
        .map_err(|_| ApiError::BadRequest("Invalid public key format".to_string()))?;
    
    // Decode the signature
    let signature_bytes = signature.from_base58()
        .map_err(|_| ApiError::BadRequest("Invalid signature encoding".to_string()))?;
    
    let signature = Ed25519Signature::try_from(signature_bytes.as_slice())
        .map_err(|_| ApiError::BadRequest("Invalid signature format".to_string()))?;
    
    // Verify the signature
    public_key.verify(message.as_bytes(), &signature)
        .map_err(|_| ApiError::Unauthorized("Signature verification failed".to_string()))?;
    
    Ok(true)
}

// This is a simplified approach for demo purposes
// In production, you'd use a proper wallet adapter
pub fn get_keypair_from_signature(public_key: &str) -> Result<Keypair, ApiError> {
    // This is just a placeholder implementation
    // In a real application, you would have a more secure way to get the keypair
    // based on the authenticated wallet
    
    // WARNING: This is not secure for production use!
    // It generates a deterministic keypair from the public key
    // just for demonstration purposes
    let seed = Pubkey::from_str(public_key)
        .map_err(|_| ApiError::BadRequest("Invalid public key".to_string()))?
        .to_bytes();
    
    let keypair = keypair_from_seed(&seed)
        .map_err(|_| ApiError::InternalServerError("Failed to generate keypair".to_string()))?;
    
    Ok(keypair)
}