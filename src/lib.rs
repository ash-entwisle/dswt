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
// static TM_INSTANCE: OnceCell<RwLock<TokenManager>> = OnceCell::new();


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

    // This is not needed, singletons arent necessary for this lib, you should impl yr own. 
    // pub fn new_singleton(
    //     alg: Option<Algorithm>,
    //     key: Option<String>,
    // ) -> &'static RwLock<TokenManager> {
        
    //     let instance: &RwLock<TokenManager> = TM_INSTANCE.get().unwrap_or_else(|| {

    //         let instance = RwLock::new(
    //             TokenManager::new(alg, key)
    //         );

    //         TM_INSTANCE.set(instance).unwrap();
    //         TM_INSTANCE.get().unwrap()
    //     });

    //     instance
    // }

    pub fn set_key(&mut self, key: Option<String>) {
        self.key = key.unwrap_or_else(|| TokenManager::gen_key());
    }

    pub fn gen_key() -> String {
        let mut rng = rand::thread_rng();
        let key: [u8; 32] = rng.gen();
        BASE64_STANDARD.encode(&key)
    }

    pub fn create_token(&self, payload: Vec<payload::PayloadItem>, typed: bool) -> Token {
        todo!()
    }

    pub fn validate_token(&self, token: &Token) -> bool {
        todo!()
    }
}
