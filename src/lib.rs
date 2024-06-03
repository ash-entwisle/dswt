use base64::prelude::*;
use once_cell::sync::OnceCell;
use serde::{Serialize, Deserialize};
use sha2::Sha256;
use hmac::{Hmac, Mac};
use token::Token;
use std::sync::RwLock;
use rand::prelude::*;

pub mod payload;
pub mod token;

static TM_INSTANCE: OnceCell<RwLock<TokenManager>> = OnceCell::new();
static VERSION: &'static str = "0.1.0";


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Algorithm {
    HS256,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenManager {
    ver: &'static str,
    key: String,
    alg: Algorithm,
    typed: bool,
}


impl TokenManager {
    pub fn new(
        alg: Option<Algorithm>,
        typed: Option<bool>,
        key: Option<String>,
    ) -> &'static RwLock<TokenManager> {
        
        let instance = TM_INSTANCE.get_or_init(|| {


            RwLock::new(TokenManager {
                ver: VERSION,
                key: key.unwrap_or_else(|| TokenManager::gen_key()),
                alg: alg.unwrap_or(Algorithm::HS256),
                typed: typed.unwrap_or(false),
            })
        });

        instance
    }

    pub fn get_instance() -> &'static RwLock<TokenManager> {
        TM_INSTANCE.get().unwrap()
    }

    pub fn set_key(&mut self, key: Option<String>) {
        self.key = key.unwrap_or_else(|| TokenManager::gen_key());
    }

    pub fn gen_key() -> String {
        let mut rng = rand::thread_rng();
        let key: [u8; 32] = rng.gen();
        BASE64_STANDARD.encode(&key)
    }



    pub fn create_token(&self, payload: Vec<payload::PayloadItem>) -> token::Token {
        todo!()
    }

    pub fn validate_token(&self, token: &token::Token) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use payload::{PayloadItem, PayloadType};

    use super::*;
    
    #[test]
    fn test_new_token_manager() {
        let tm = TokenManager::new(None, None, None).read().unwrap();
        assert_eq!(tm.ver, VERSION);
        assert_eq!(tm.alg, Algorithm::HS256);
        assert_eq!(tm.typed, false);
    }
    
    #[test]
    fn test_set_key() {
        let mut tm = TokenManager::new(None, None, None).write().unwrap();
        let old_key = tm.key.clone();
        tm.set_key(Some(String::from("new_key")));
        assert_ne!(tm.key, old_key);
        assert_eq!(tm.key, "new_key");
    }
    
    #[test]
    fn test_gen_key() {
        let key = TokenManager::gen_key();
        assert_eq!(key.len(), 44); // Check if key is generated correctly
    }
    
    #[test]
    fn test_create_token() {
        let tm = TokenManager::new(None, None, None).read().unwrap();
        let payload = vec![
            PayloadItem::new("name", "John", PayloadType::String),
            PayloadItem::new("age", 30, PayloadType::Int),
        ];
        let token = tm.create_token(payload);

        // Add assertions to check if the token is created correctly
        todo!();
    }
    
    #[test]
    fn test_validate_token() {
        let tm = TokenManager::new(None, None, None).read().unwrap();
        let payload = vec![
            payload::PayloadItem::new("name", "John", payload::PayloadType::String),
            payload::PayloadItem::new("age", 30, payload::PayloadType::Int),
        ];
        let token = tm.create_token(payload);
        let is_valid = tm.validate_token(&token);
        // Add assertions to check if the token is validated correctly
        todo!();
    }
}