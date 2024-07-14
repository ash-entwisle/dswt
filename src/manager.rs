use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Algorithm, Token, VERSION};

/// A struct that manages the creation and validation of tokens created with a given key and algorithm
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenManager {

    #[doc(hidden)]
    pub version: u8,

    /// Algorithm used to hash the payload
    pub algorithm: Algorithm,

    key: String,
}


impl TokenManager {

    /// Function to create a new TokenManager

    pub fn new(alg: Algorithm, key: &str) -> TokenManager {
        TokenManager {
            version: VERSION,
            key: key.to_string(),
            algorithm: alg,
        }
    }

    /// Create a new token``
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