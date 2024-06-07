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

        if parts.len() != 2 {
            return Err("Invalid payload item".to_string());
        }

        let key = parts[0].to_string();
        let value = parts[1].to_string();

        Ok(PayloadItem { key, value })

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payload_item_new() {
        let item = PayloadItem::new("name", "John");
        assert_eq!(item.key, "name");
        assert_eq!(item.value, "John");
    }

    #[test]
    fn test_payload_item_display() {
        let item = PayloadItem::new("age", 25);
        assert_eq!(item.to_string(), "age=25");
    }

    #[test]
    fn test_payload_item_from_str() {
        let item: PayloadItem = "city=New York".parse().unwrap();
        assert_eq!(item.key, "city");
        assert_eq!(item.value, "New York");
    }

    #[test]
    fn test_payload_item_from_str_invalid() {
        let item: Result<PayloadItem, _> = "invalid".parse();
        assert!(item.is_err());
    }
}
