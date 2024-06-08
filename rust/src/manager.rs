// Crate imports
use base64::prelude::*;
use rand::prelude::*;

// std imports
use std::collections::HashMap;

use crate::{Algorithm, Token, VERSION};

/// A struct that manages the creation and validation of tokens created with a given key and algorithm
#[derive(Debug, Clone, PartialEq)]
pub struct TokenManager {

    #[doc(hidden)]
    pub version: u8,

    /// Algorithm used to hash the payload
    pub algorithm: Algorithm,

    key: String,
}


impl TokenManager {

    /// Function to create a new TokenManager
    /// 
    /// ```rust
    /// let token_manager = TokenManager::new(
    ///     Algorithm::HS256, 
    ///     "your_key" 
    /// );
    pub fn new(alg: Algorithm, key: &str) -> TokenManager {
        TokenManager {
            version: VERSION,
            key: key.to_string(),
            algorithm: alg,
        }
    }

    /// Create a new token
    /// 
    /// ```rust
    /// let token = token_manager.create_token(payload_as_hashmap);
    pub fn create_token(&self, payload: HashMap<String, String>) -> Token {
        Token::new(
            self.algorithm.clone(), 
            payload, 
            &self.key
        )
    }

    /// Validate a token
    pub fn validate_token(&self, token: &Token) -> bool {
        token.hash == token.get_hash(&self.key)
    }
}