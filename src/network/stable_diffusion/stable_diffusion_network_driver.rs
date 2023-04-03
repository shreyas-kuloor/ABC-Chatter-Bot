use std::env;
use log::info;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::network::{
    models::network_models::{
        NetworkResult,
    },
};

pub struct StableDiffusionClient {
    base_url: String,
    client: reqwest::Client,
}

fn create_client() -> reqwest::Client {
    let client = reqwest::Client::new();
    client
}

impl StableDiffusionClient {
    pub fn new() -> Self {
        Self {
            base_url: env::var("STABLE_DIFFUSION_BASE_URL").unwrap(),
            client: create_client(),
        }
    }

    pub async fn send_request<T: for<'a> Deserialize<'a> + Serialize + std::fmt::Debug, U: for<'a> Deserialize<'a> + Serialize + std::fmt::Debug>(&self, path: String, method: Method, request: Option<T>) -> NetworkResult<U> {
        let url = format!("{}/{}", &self.base_url, path);
        
        info!("Stable Diffusion request body: {:?}", &request);

        let call = match request {
            Some(populated_request) => self.client.request(method.clone(), &url).json(&populated_request),
            None => self.client.request(method.clone(), &url),
        };

        let response = call
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let parsed_response = response.json::<U>().await?;
                info!("Stable Diffusion response body: {:?}", &parsed_response);
                Ok(parsed_response)
            },
            _ => {
                panic!("Unexpected response: {:?}", response.json().await?);
            }
        }
    }
}
