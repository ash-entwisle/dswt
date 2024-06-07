// Crate imports
use base64::prelude::*;
use once_cell::sync::OnceCell;
use serde::{Serialize, Deserialize};
use std::sync::RwLock;
use rand::prelude::*;

// Module declarations
pub mod algorithms;
pub mod payload;
pub mod token;

// Local imports
use token::Token;
use algorithms::Algorithm;
use payload::PayloadItem;

pub static VERSION: &'static str = "0.1.0";


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenManager {
    ver: &'static str,
    key: String,
    alg: Algorithm,
}


impl TokenManager {

    pub fn new(alg: Option<Algorithm>, key: Option<String>) -> TokenManager {
        TokenManager {
            ver: VERSION,
            key: key.unwrap_or_else(|| TokenManager::gen_key()),
            alg: alg.unwrap_or(Algorithm::HS256),
        }
    }

    pub fn set_key(&mut self, key: Option<String>) {
        self.key = key.unwrap_or_else(|| TokenManager::gen_key());
    }

    pub fn gen_key() -> String {
        let mut rng = rand::thread_rng();
        let key: [u8; 32] = rng.gen();
        BASE64_STANDARD.encode(&key)
    }

    pub fn create_token(&self, payload: Vec<payload::PayloadItem>, typed: bool) -> Token {
        Token::new(
            self.ver, 
            self.alg.clone(), 
            payload, &self.key
        )
    }

    pub fn validate_token(&self, token: &Token) -> bool {
        token.hash == token.get_hash(&self.key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_token_manager_with_alg_and_key() {
        let alg = Algorithm::HS256;
        let key = "secret_key".to_string();
        let token_manager = TokenManager::new(
            Some(alg.clone()), 
            Some(key.clone())
        );
        
        assert_eq!(token_manager.ver, VERSION);
        assert_eq!(token_manager.alg, alg);
        assert_eq!(token_manager.key, key);
    }
    
    #[test]
    fn test_new_token_manager_with_default_alg_and_key() {
        let token_manager = TokenManager::new(None, None);
        
        assert_eq!(token_manager.ver, VERSION);
        assert_eq!(token_manager.alg, Algorithm::HS256);
        assert_ne!(token_manager.key, "");
    }
    
    #[test]
    fn test_set_key_with_existing_key() {
        let mut token_manager = TokenManager::new(None, Some("existing_key".to_string()));
        let existing_key = token_manager.key.clone();
        
        token_manager.set_key(Some("new_key".to_string()));
        
        assert_eq!(token_manager.key, "new_key");
        assert_ne!(token_manager.key, existing_key);
    }
    
    #[test]
    fn test_set_key_without_existing_key() {
        let mut token_manager = TokenManager::new(None, None);
        
        token_manager.set_key(Some("new_key".to_string()));
        
        assert_eq!(token_manager.key, "new_key");
    }
    
    #[test]
    fn test_gen_key() {
        let key = TokenManager::gen_key();
        
        assert_ne!(key, "");
    }
    
    #[test]
    fn test_create_token() {
        let token_manager = TokenManager::new(None, None);
        let payload = vec![
            PayloadItem::new("name", "John"),
            PayloadItem::new("age", "30"),
        ];
        
        let token = token_manager.create_token(payload.clone(), true);
        let ver_char = VERSION.chars().next().unwrap();

        assert_eq!(token.version, ver_char);
        assert_eq!(token.algorithm, Algorithm::HS256);
        assert_eq!(token.payload, payload);
        assert_ne!(token.hash, "");
    }
    
    #[test]
    fn test_validate_token_valid() {
        let token_manager = TokenManager::new(None, None);
        let payload = vec![
            PayloadItem::new("name", "John"),
            PayloadItem::new("age", "30"),
        ];
        let token = token_manager.create_token(payload.clone(), true);
        
        let is_valid = token_manager.validate_token(&token);
        
        assert_eq!(is_valid, true);
    }
    
    #[test]
    fn test_validate_token_invalid() {
        let token_manager = TokenManager::new(None, None);
        let payload = vec![
            PayloadItem::new("name", "John"),
            PayloadItem::new("age", "30"),
        ];
        let mut token = token_manager.create_token(payload.clone(), true);
        token.hash = "invalid_hash".to_string();
        
        let is_valid = token_manager.validate_token(&token);
        
        assert_eq!(is_valid, false);
    }
}