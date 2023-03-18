use reqwest::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum NetworkErrorType {
    TokenQuotaReached,
    Unknown
}

#[derive(Debug)]
pub struct NetworkError {
    pub error_type: NetworkErrorType,
}

impl NetworkError{
    pub fn new(error_type: NetworkErrorType) -> Self {
        Self {
            error_type,
        }
    }
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.error_type {
            NetworkErrorType::TokenQuotaReached => write!(f, "Token quota was reached."),
            NetworkErrorType::Unknown => write!(f, "A network error occurred."),
        }
        
    }
}

impl From<Error> for NetworkError {
    fn from(_err: Error) -> Self {
        NetworkError::new(NetworkErrorType::Unknown)
    }
}