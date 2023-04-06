use std::env;
use chrono::DateTime;
use log::info;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::errors::network_error::{
    NetworkError,
    NetworkErrorType,
};
use crate::network::{
    models::network_models::{
        NetworkResult,
        BearerToken,
    },
};

pub struct OpenAIClient {
    base_url: String,
    bearer_token: BearerToken,
    client: reqwest::Client,
}

fn create_client() -> reqwest::Client {
    let client = reqwest::Client::new();
    client
}

impl OpenAIClient {
    pub fn new() -> Self {
        Self {
            base_url: env::var("OPENAI_BASE_URL").unwrap(),
            bearer_token: BearerToken::new(env::var("OPENAI_API_KEY").unwrap(), DateTime::default()),
            client: create_client(),
        }
    }

    pub async fn send_request<T: for<'a> Deserialize<'a> + Serialize + std::fmt::Debug, U: for<'a> Deserialize<'a> + Serialize + std::fmt::Debug>(&self, path: String, method: Method, request: Option<T>) -> NetworkResult<U> {
        let url = format!("{}/{}", &self.base_url, path);
        
        info!("OpenAI request body: {:?}", &request);

        let call = match request {
            Some(populated_request) => self.client.request(method.clone(), &url).bearer_auth(&self.bearer_token).json(&populated_request),
            None => self.client.request(method.clone(), &url).bearer_auth(&self.bearer_token),
        };

        let response = call
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let parsed_response = response.json::<U>().await?;
                info!("OpenAI response body: {:?}", &parsed_response);
                Ok(parsed_response)
            },
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                Err(NetworkError::new(NetworkErrorType::TokenQuotaReached, None))
            },
            _ => {
                panic!("Unexpected response: {:?}", response.json().await?);
            }
        }
    }
}
