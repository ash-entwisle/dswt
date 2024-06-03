use serde::{Serialize, Deserialize};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PayloadType {
    UUID,
    String,
    Int,
    Float,
    Bool,
}

impl std::fmt::Display for PayloadType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PayloadType::UUID => write!(f, "uuid"),
            PayloadType::String => write!(f, "string"),
            PayloadType::Int => write!(f, "int"),
            PayloadType::Float => write!(f, "float"),
            PayloadType::Bool => write!(f, "bool"),
        }
    }
}

impl FromStr for PayloadType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "uuid" => Ok(PayloadType::UUID),
            "string" => Ok(PayloadType::String),
            "int" => Ok(PayloadType::Int),
            "float" => Ok(PayloadType::Float),
            "bool" => Ok(PayloadType::Bool),
            _ => Err(format!("Invalid PayloadType: {}", s)),
        }
    }
}   
