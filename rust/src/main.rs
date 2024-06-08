use std::collections::HashMap;

use dswt::{Algorithm, Token, TokenManager};

fn main() {

    let payload: HashMap<String, String> = [
        ("key1".to_string(), "value1".to_string()),
        ("key2".to_string(), "value2".to_string()),
        ("key3".to_string(), "value3".to_string()),
    ].iter().cloned().collect();

    let token_manager = TokenManager::new(
        Algorithm::HS256, 
        "your_key"
    );

    let token: Token = token_manager.create_token(payload);

    let token_str = token.to_string();
    println!("{}", token_str);

    let token2: Token = token_str.parse().unwrap();

    
}