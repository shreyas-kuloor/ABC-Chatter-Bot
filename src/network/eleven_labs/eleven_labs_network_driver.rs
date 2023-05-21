use std::{env, time::Duration};
use bytes::Bytes;
use log::info;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::network::{
    models::network_models::{
        NetworkResult,
    },
};

pub struct ElevenLabsClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

fn create_client() -> reqwest::Client {
    let client = reqwest::Client::new();
    client
}

impl ElevenLabsClient {
    pub fn new() -> Self {
        Self {
            api_key: env::var("ELEVEN_LABS_API_KEY").unwrap(),
            base_url: env::var("ELEVEN_LABS_BASE_URL").unwrap(),
            client: create_client(),
        }
    }

    pub async fn send_request_bytes<T: for<'a> Deserialize<'a> + Serialize + std::fmt::Debug>(&self, path: String, method: Method, request: Option<T>) -> NetworkResult<Bytes> {
        let url = format!("{}/{}", &self.base_url, path);
        
        info!("Eleven labs request body: {:?}", &request);

        let request_with_headers = self.client.request(method.clone(), &url).header("xi-api-key", self.api_key.clone());

        let call = match request {
            Some(populated_request) => request_with_headers.json(&populated_request),
            None => request_with_headers,
        };

        let response = call
            .timeout(Duration::from_secs(600))
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let parsed_response = response.bytes().await?;
                Ok(parsed_response)
            },
            _ => {
                panic!("Unexpected response: {:?}", response.json().await?);
            }
        }
    }

    pub async fn send_request<U: for<'a> Deserialize<'a> + Serialize + std::fmt::Debug>(&self, path: String, method: Method) -> NetworkResult<U> {
        let url = format!("{}/{}", &self.base_url, path);
        
        let response = self.client.request(method.clone(), &url)
            .header("xi-api-key", self.api_key.clone())
            .timeout(Duration::from_secs(600))
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let parsed_response = response.json::<U>().await?;
                Ok(parsed_response)
            },
            _ => {
                panic!("Unexpected response: {:?}", response.json().await?);
            }
        }
    }
}
