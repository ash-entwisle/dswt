use std::fmt::Display;

use base64::prelude::*;
use serde::{Serialize, Deserialize};
use sha2::Sha256;
use hmac::{Hmac, Mac};
use rand::prelude::*;

use crate::payload;
use crate::types::PayloadType;
use crate::algorithms::Algorithm;


/*
    Tokens are in the format:
    header;payload;hash

    header is DSWT/<alg> where alg is the algorithm used to hash the payload
    this is base64 encoded
    
    payload is a csv of every item after index 0 in the header 
    in the format: `key1:type=value,key2:type=value,...`
    This is base64 encoded

    hash is the base64 encoded hash of the header and payload
*/


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {

    // base64 encoded header
    // gets formatted into DSWT-<version>/<algorithm>
    pub version: char,
    pub algorithm: Algorithm,

    // base64 encoded payload
    // gets formatted into key:type=value,key:type=value,...
    // if typed is false, then it is formatted into key=value,key=value,...
    pub payload: Vec<payload::PayloadItem>,

    // base64 encoded hash of the header and payload
    pub hash: String,
}

impl Token {
    pub fn new(
        version: &'static str,
        algorithm: Algorithm,
        payload: Vec<payload::PayloadItem>, 
        key: String
    ) -> Self {

        let mut token = Token { 
            version: version.chars().next().unwrap(),
            algorithm,
            payload,
            hash: "".to_string(),
        };

        token.set_hash(key);
        token
    }

    fn set_hash(&mut self, key: String) {

        let to_hash = format!("{};{}",
            self.to_str_header(),
            self.to_str_payload()
        );

        let hash: String = match self.algorithm {
            Algorithm::HS256 => {
                let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes()).unwrap();
                mac.update(to_hash.as_bytes());
                BASE64_STANDARD.encode(&mac.finalize().into_bytes())
            }
        };

        self.hash = hash;
    }

    fn to_str_header(&self) -> String {
        
        let fmt = format!("DSWT-{}/{}", 
            self.version, 
            self.algorithm
        );

        BASE64_STANDARD.encode(&fmt)
    }

    fn to_str_payload(&self) -> String {
        
        let payload_str = self.payload.iter()
            .map(|item| item.to_string())
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

