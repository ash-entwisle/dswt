use serde::{Serialize, Deserialize};

use std::fmt::Display;
use std::str::FromStr;

use crate::types::PayloadType;



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayloadItem {
    pub key: String,
    pub value: String,
    pub ptype: PayloadType,
}

impl PayloadItem {
    pub fn new<T: Display>(key: &str, value: T, ptype: PayloadType) -> Self {
        PayloadItem { key: key.to_string(), value: value.to_string(), ptype }
    }
}

impl Display for PayloadItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}={}", 
            self.key, 
            self.ptype, 
            self.value.to_string()
        )
    }
}

impl FromStr for PayloadItem {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // in the format key:type=value
        
        let parts: Vec<&str> = s.split('=').collect();
        let value = parts[1].to_string();

        let key_type: Vec<&str> = parts[0].split(':').collect();
        let key = key_type[0].to_string();
        let ptype = key_type[1].parse().unwrap();

        Ok(PayloadItem { key, value, ptype })

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payload_type_display() {
        assert_eq!(format!("{}", PayloadType::UUID), "uuid");
        assert_eq!(format!("{}", PayloadType::String), "string");
        assert_eq!(format!("{}", PayloadType::Int), "int");
        assert_eq!(format!("{}", PayloadType::Float), "float");
        assert_eq!(format!("{}", PayloadType::Bool), "bool");
    }

    #[test]
    fn test_payload_item_new() {
        let item = PayloadItem::new("key", "value", PayloadType::String);
        assert_eq!(item.key, "key");
        assert_eq!(item.value, "value");
        assert_eq!(item.ptype, PayloadType::String);
    }

    #[test]
    fn test_payload_item_display() {
        let item = PayloadItem::new("key", "value", PayloadType::String);
        assert_eq!(format!("{}", item), "key:string=value");
    }
}


