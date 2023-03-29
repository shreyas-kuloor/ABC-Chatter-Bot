use std::fmt::Display;
use serde::{
    Serialize, 
    Deserialize
};
use chrono::{DateTime, Utc};
use crate::errors::network_error::NetworkError;

pub type NetworkResult<T> = std::result::Result<T, NetworkError>;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    token_type: String,
}

pub struct BearerToken {
    pub token: String,
    pub expiration: DateTime<Utc>,
}

impl BearerToken {
    pub fn new(access_token: String, expiration: DateTime<Utc>) -> Self {
        Self {
            token: access_token,
            expiration,
        }
    }
}

impl Display for BearerToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.token, f)
    }
}
