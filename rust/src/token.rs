use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use base64::prelude::*;
use sha2::Sha256;
use hmac::{Hmac, Mac};

use crate::Algorithm;



/// A struct that holds information about a Delimeter Separated Web Token. 
/// It is not reccomended to create a token directly using this struct,
/// instead create one through the `TokenManager` struct.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {

    /// version of the token, this is automatically set to the crate version and should not be changed
    pub version: u8,
    
    /// Te algorithm used to hash the payload
    pub algorithm: Algorithm,

    /// payload of the token stored in a hashmap as key-value pairs
    pub payload: HashMap<String, String>,

    /// hash of the token that gets generated when the token is created, it is used to verify the token is valid
    /// this is automatically set when the token is created and should not be changed
    pub hash: String,
}

impl Token {

    /// Create a new token with the given algorithm, payload, and key
    /// 
    /// Example:
    /// ```rust
    /// let payload: HashMap<String, String> = [
    ///     ("key1".to_string(), "value1".to_string()),
    ///     ("key2".to_string(), "value2".to_string()),
    ///     ("key3".to_string(), "value3".to_string()),
    /// ].iter().cloned().collect();
    /// 
    /// let token = Token::new(
    ///     Algorithm::HS256, // or any other algorithm
    ///     payload,
    ///     "your_key_here".to_string()
    /// );
    /// ```
    pub fn new(
        algorithm: Algorithm,
        payload: HashMap<String, String>,
        key: &str
    ) -> Self {

        let mut token = Token { 
            version: crate::VERSION,
            algorithm,
            payload,
            hash: "".to_string(),
        };

        token.set_hash(key);
        token
    }
    
    /// Get the hash of the token using the given key
    /// 
    /// Example:
    /// ```rust
    /// let token = Token::new(
    ///     Algorithm::HS256, // or any other algorithm
    ///     payload,
    ///     "your_key_here".to_string()
    /// );
    /// 
    /// let hash = token.get_hash("your_key_here");
    /// 
    /// assert_eq!(hash, token.hash);
    /// ```
    pub fn get_hash(&self, key: &str) -> String {

        let to_hash = format!("{};{}",
            self.to_str_header(),
            self.to_str_payload()
        );

        match self.algorithm {
            Algorithm::HS256 => {
                let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes()).unwrap();
                mac.update(to_hash.as_bytes());
                BASE64_STANDARD.encode(&mac.finalize().into_bytes())
            }
        }
    }

    fn set_hash(&mut self, key: &str) {
        self.hash = self.get_hash(key);
    }

}

impl Token {
    fn to_str_header(&self) -> String {
        
        let fmt = format!("DSWT-{}/{}", 
            self.version, 
            self.algorithm
        );

        BASE64_STANDARD.encode(&fmt)
    }

    fn to_str_payload(&self) -> String {
        
        let payload_str = self.payload.iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<String>>()
            .join(",");
        
        BASE64_STANDARD.encode(&payload_str)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{};{};{}", 
            self.to_str_header(), 
            self.to_str_payload(), 
            self.hash
        )
    }
}

impl FromStr for Token {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        let parts: Vec<&str> = s.split(';').collect();

        let header = parts[0];
        let payload = parts[1];
        let hash = parts[2];

        let header = BASE64_STANDARD.decode(header).unwrap();
        let payload = BASE64_STANDARD.decode(payload).unwrap();

        let header = String::from_utf8(header).unwrap();
        let payload = String::from_utf8(payload).unwrap();

        let header = header.split('/').collect::<Vec<&str>>();
        let version = header[0].chars().nth(5).unwrap();
        let algorithm = Algorithm::from(header[1]);

        let payload = payload.split(',').collect::<Vec<&str>>()
            .iter()
            .map(|item| {
                let item = item.split('=').collect::<Vec<&str>>();
                (item[0].to_string(), item[1].to_string())
            })
            .collect::<HashMap<String, String>>();

        Ok(Token {
            version: version.to_digit(10).unwrap() as u8,
            algorithm,
            payload,
            hash: hash.to_string(),
        })
    }
}

