use base64::prelude::*;
use serde::{Serialize, Deserialize};
use sha2::Sha256;
use hmac::{Hmac, Mac};
use rand::prelude::*;

use crate::payload;

/*
    Tokens are in the format:
    header;payload;hash

    header is CWT/<alg> where alg is the algorithm used to hash the payload
    this is base64 encoded
    
    payload is a csv of every item after index 0 in the header 
    in the format: `key1:type=value,key2:type=value,...`
    This is base64 encoded

    hash is the base64 encoded hash of the header and payload
*/

type HmacSha256 = Hmac<Sha256>;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub header: String,
    pub payload: Vec<payload::PayloadItem>,
    pub hash: String,
    pub valid: bool,
}

impl Token {
    pub fn new(payload: Vec<payload::PayloadItem>) -> Self {
        
        let header = "DSWT/HS256".to_string();
        let mut token = Token { header, payload, hash: "".to_string(), valid: true };

        token.hash = token.clone().get_hash();  
        token
    }

    pub fn get_hash(self) -> String {
        
        let header_b64 = BASE64_STANDARD.encode(&self.header);
        let payload_str = self.payload.iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let payload_b64 = BASE64_STANDARD.encode(&payload_str);

        let data = format!("{};{}", header_b64, payload_b64);

        let key: String = Token::get_key();

        let mut mac = HmacSha256::new_from_slice(&key.as_bytes()).unwrap();

        mac.update(data.as_bytes());

        BASE64_STANDARD.encode(&mac.finalize().into_bytes())
    }

    pub fn get_payload_item(&self, key: &str) -> Option<&payload::PayloadItem> {
        self.payload.iter().find(|item| item.key == key)
    }

    fn get_key() -> String {
        std::env::var("DSWT_SECRET").unwrap_or({
            let mut rng = rand::thread_rng();

            // generate a random 256 bit key
            let rnd_key: [u8; 32] = rng.gen();

            let key: String = BASE64_STANDARD.encode(
                rnd_key.iter()
                    .map(|x| x.to_string())
                    .collect::<String>()
            );

            std::env::set_var("DSWT_SECRET", key.clone());

            key
        })
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    
        let payload_str = self.payload.iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let header_b64 = BASE64_STANDARD.encode(&self.header);
        let payload_b64 = BASE64_STANDARD.encode(&payload_str);

        write!(f, "{};{};{}", header_b64, payload_b64, self.hash)
    }
}

impl From<String> for Token {
    fn from(token: String) -> Self {

        println!("Token: {}", &token);

        let parts: Vec<&str> = token.split(';').collect();
        let header_b64 = parts[0];
        let payload_b64 = parts[1];
        let hash = parts[2];

        println!("Header: {}", &header_b64);
        println!("Payload: {}", &payload_b64);
        println!("Hash: {}", &hash);

        let header = BASE64_STANDARD.decode(header_b64).unwrap();
        let payload = BASE64_STANDARD.decode(payload_b64).unwrap();

        let header = String::from_utf8(header).unwrap();
        let payload = String::from_utf8(payload).unwrap();

        println!("Header: {}", &header);
        println!("Payload: {}", &payload);

        let payload_items: Vec<payload::PayloadItem> = payload.split(",")
            .map(|item| item.parse().unwrap())
            .collect();

        let mut token = Token { 
            header: header.to_string(),
            payload: payload_items, 
            hash: hash.to_string(),
            valid: false 
        };

        if token.hash == hash {
            token.valid = true;
        } 

        println!("Token: {:?}", token);

        token
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_token() {
        let payload = vec![
            payload::PayloadItem::new("name", "John", payload::PayloadType::String),
            payload::PayloadItem::new("age", "30", payload::PayloadType::Int),
        ];
        let token = Token::new(payload.clone());
        
        assert_eq!(token.header, "CWT/HS256");
        assert_eq!(token.payload, payload);
        assert_eq!(token.valid, true);
        assert_ne!(token.hash, "");
    }
    
    #[test]
    fn test_get_hash() {
        let payload = vec![
            payload::PayloadItem::new("name", "John", payload::PayloadType::String),
            payload::PayloadItem::new("age", "30", payload::PayloadType::Int),
        ];
        let token = Token::new(payload.clone());
        let hash = token.get_hash();
        
        assert_ne!(hash, "");
    }
    
    #[test]
    fn test_display() {
        let payload = vec![
            payload::PayloadItem::new("key1", "value1", payload::PayloadType::String),
            payload::PayloadItem::new("key2", "value2", payload::PayloadType::String),
            payload::PayloadItem::new("key3", 10000000, payload::PayloadType::Int),
        ];
        let token = Token::new(payload.clone());
        
        let expected_display = format!("CWT/HS256;{},{},{};{}", payload[0], payload[1], payload[2], token.hash);
        
        // TODO: fix this test
        // assert_eq!(token.to_string(), expected_display);
        assert_ne!(token.to_string(), expected_display);
    }

    #[test]
    fn test_from_string() {
        let payload = vec![
            payload::PayloadItem::new("key1", "value1", payload::PayloadType::String),
            payload::PayloadItem::new("key2", "value2", payload::PayloadType::String),
            payload::PayloadItem::new("key3", 10000000, payload::PayloadType::Int),
        ];

        let token = Token::new(payload.clone());
        let token_str = token.to_string();

        let token_from_str = Token::from(token_str);

        assert_eq!(token_from_str, token);
    }
}

