#![warn(missing_docs)]

//! # Token Manager

// Crate imports
use base64::prelude::*;
use serde::{Serialize, Deserialize};
use rand::prelude::*;

// std imports
use std::collections::HashMap;

// Local imports
use token::Token;
use algorithms::Algorithm;

/// Base64 encoding standard
pub mod algorithms;

/// Algorithms used to hash the payload
pub mod token;


/// Version of the library
pub static VERSION: u8 = 1;


/// Token Manager
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenManager {
    version: u8,
    key: String,
    algorithm: Algorithm,
}


impl TokenManager {

    /// Create a new TokenManager
    pub fn new(alg: Option<Algorithm>, key: Option<String>) -> TokenManager {
        TokenManager {
            version: VERSION,
            key: key.unwrap_or_else(|| TokenManager::gen_key()),
            algorithm: alg.unwrap_or(Algorithm::HS256),
        }
    }

    /// Set the key for the TokenManager
    pub fn set_key(&mut self, key: Option<String>) {
        self.key = key.unwrap_or_else(|| TokenManager::gen_key());
    }

    /// Set the algorithm for the TokenManager
    pub fn gen_key() -> String {
        let mut rng = rand::thread_rng();
        let key: [u8; 32] = rng.gen();
        BASE64_STANDARD.encode(&key)
    }

    /// Create a new token
    pub fn create_token(&self, payload: HashMap<String, String>) -> Token {
        Token::new(
            self.version,
            self.algorithm.clone(), 
            payload, &self.key
        )
    }

    /// Validate a token
    pub fn validate_token(&self, token: &Token) -> bool {
        token.hash == token.get_hash(&self.key)
    }
}
