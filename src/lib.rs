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
pub static VERSION: &'static str = "0.1.0";


/// Token Manager
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenManager {
    ver: &'static str,
    key: String,
    alg: Algorithm,
}


impl TokenManager {

    /// Create a new TokenManager
    pub fn new(alg: Option<Algorithm>, key: Option<String>) -> TokenManager {
        TokenManager {
            ver: VERSION,
            key: key.unwrap_or_else(|| TokenManager::gen_key()),
            alg: alg.unwrap_or(Algorithm::HS256),
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
            self.ver, 
            self.alg.clone(), 
            payload, &self.key
        )
    }

    /// Validate a token
    pub fn validate_token(&self, token: &Token) -> bool {
        token.hash == token.get_hash(&self.key)
    }
}
