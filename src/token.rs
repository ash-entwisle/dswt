use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use base64::prelude::*;
use serde::{Serialize, Deserialize};
use sha2::Sha256;
use hmac::{Hmac, Mac};

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
    // pub payload: Vec<payload::PayloadItem>,
    pub payload: HashMap<String, String>,

    // base64 encoded hash of the header and payload
    pub hash: String,
}

impl Token {
    pub fn new(
        version: &'static str,
        algorithm: Algorithm,
        payload: HashMap<String, String>,
        key: &str
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
            version,
            algorithm,
            payload,
            hash: hash.to_string(),
        })
    }
}

