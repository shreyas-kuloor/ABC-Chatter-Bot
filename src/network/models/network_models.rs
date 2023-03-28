use std::fmt::Display;

use crate::errors::network_error::NetworkError;

pub type NetworkResult<T> = std::result::Result<T, NetworkError>;

pub struct BearerToken {
    pub token: String,
}

impl BearerToken {
    pub fn new(access_token: String) -> Self {
        Self {
            token: access_token.into(),
        }
    }
}

impl Display for BearerToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(&self.token, f)
    }
}
