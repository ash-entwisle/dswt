#![warn(missing_docs)]

//! # Delimiter-Separated Web Tokens
//! 
//! I got bored and decided to make my own token format. 
//! This is a simple token format that uses delimiters to separate the header, payload, and hash of the token
//! Instead of using something like JSON (like JWT's).
//! 
//! ## Usage
//! 
//! To use DSWT, you need to create a `TokenManager` with the algorithm you want to use and the key you want to use.
//! You then need a payload that you want to encode into a token.
//! This is a hashmap of key-value pairs where the key is a string and the value is a string.
//! You then create a token with the `create_token` method on the `TokenManager` and pass in the payload.
//! This will then return your token of type `Token`.
//! 
//! ```rust
//! use std::collections::HashMap;
//! 
//! use dswt::{Algorithm, Token, TokenManager};
//! 
//! fn main() {
//! 
//!     let payload: HashMap<String, String> = [
//!         ("key1".to_string(), "value1".to_string()),
//!         ("key2".to_string(), "value2".to_string()),
//!         ("key3".to_string(), "value3".to_string()),
//!     ].iter().cloned().collect();
//! 
//!     let token_manager = TokenManager::new(
//!         Algorithm::HS256, 
//!         "your_key"
//!     );
//! 
//!     let token: Token = token_manager.create_token(payload);
//! 
//!     let token_str = token.to_string();
//!     println!("{}", token_str);
//! 
//!     let token2: Token = token_str.parse().unwrap();
//! 
//!     
//! }
//! ```
//! 
//! ## Format
//! 
//! The overall structure of a DSWT token is `<header>;<payload>;<hash>`. 
//! Each part is a base64 encoded string representing a different part of the token.
//! Each part is separated by a semicolon `;`.
//! 
//! ### Header
//! 
//! The header is the first part of the token, 
//! It holds information about the token such as the version and the algorithm used to hash the payload.
//! It is encoded in base64, and is in the format `DSWT-<ver>/<alg>`.
//! Where `<ver>` is the version of the token and `<alg>` is the algorithm used to hash the payload.
//! 
//! ### Payload
//! 
//! The payload is the second part of the token,
//! It holds the data that the token is supposed to represent.
//! It is encoded in base64, and is in the format `key1=value,key2=value,...`.
//! Each key-value pair is separated by a comma `,`.
//! 
//! ### Hash
//! 
//! The hash is the last part of the token,
//! It is the hash of the header and payload, and is used to verify the token is valid.
//! 
//! ## Full Token Example
//! 
//! To give a full example of what a DSWT token would look like, 
//! here is a full example token that is not base64 encoded would look like this:
//! 
//! ```plaintext
//! DSWT-<ver>/<alg>;<key1>=<value>,<key2>=<value2>;<hash>
//! ```
//! 
//! 

// module imports
mod algorithms;
mod token;
mod manager;

// Re-exports
pub use token::Token;
pub use algorithms::Algorithm;
pub use manager::TokenManager;

/// The current version of the DSWT Spec
pub static VERSION: u8 = 1;
