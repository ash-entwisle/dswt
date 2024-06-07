use serde::{Serialize, Deserialize};

use std::fmt::Display;
use std::str::FromStr;



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayloadItem {
    pub key: String,
    pub value: String,
}

impl PayloadItem {
    pub fn new<T: Display>(key: &str, value: T) -> Self {
        PayloadItem { key: key.to_string(), value: value.to_string() }
    }
}

impl Display for PayloadItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}={}", 
            self.key, 
            self.value.to_string()
        )
    }
}

impl FromStr for PayloadItem {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        let parts: Vec<&str> = s.split('=').collect();

        let key = parts[0].to_string();
        let value = parts[1].to_string();

        Ok(PayloadItem { key, value })

    }
}
