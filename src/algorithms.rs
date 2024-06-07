use serde::{Deserialize, Serialize};

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Algorithm {
    HS256,
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Algorithm::HS256 => write!(f, "HS256"),
        }
    }
}

impl Default for Algorithm {
    fn default() -> Self {
        Algorithm::HS256
    }
}

impl From<&str> for Algorithm {
    fn from(s: &str) -> Self {
        match s {
            "HS256" => Algorithm::HS256,
            _ => Algorithm::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_display() {
        assert_eq!(Algorithm::HS256.to_string(), "HS256");
    }

    #[test]
    fn test_algorithm_default() {
        let algorithm: Algorithm = Default::default();
        assert_eq!(algorithm, Algorithm::HS256);
    }

    #[test]
    fn test_algorithm_from_str_valid() {
        let algorithm: Algorithm = "HS256".into();
        assert_eq!(algorithm, Algorithm::HS256);
    }

    #[test]
    fn test_algorithm_from_str_invalid() {
        let algorithm: Algorithm = "invalid".into();
        assert_eq!(algorithm, Algorithm::HS256);
    }
}