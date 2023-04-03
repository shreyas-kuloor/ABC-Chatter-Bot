use std::{env, vec};
use chrono::{Utc, Duration};
use log::info;
use reqwest::{Body, Method};
use serde::{Deserialize, Serialize};

use crate::errors::network_error::{
    NetworkError,
    NetworkErrorType,
};
use crate::network::models::network_models::AccessTokenResponse;
use crate::network::{
    models::network_models::{
        NetworkResult,
        BearerToken,
    },
};

pub struct IGDBClient {
    base_url: String,
    bearer_token: BearerToken,
    client_id: String,
    client: reqwest::Client,
}

fn create_client() -> reqwest::Client {
    let client = reqwest::Client::new();
    client
}

impl IGDBClient {
    pub fn new() -> Self {
        Self {
            base_url: env::var("IGDB_BASE_URL").unwrap(),
            client_id: env::var("IGDB_CLIENT_ID").unwrap(),
            bearer_token: BearerToken::new(String::default(), Utc::now()),
            client: create_client(),
        }
    }

    pub async fn refresh_bearer_token(&mut self) -> NetworkResult<()> {
        let auth_url = env::var("IGDB_AUTH_URL").unwrap();
        let client_id = env::var("IGDB_CLIENT_ID").unwrap();
        let client_secret = env::var("IGDB_CLIENT_SECRET").unwrap();
        let grant_type = String::from("client_credentials");

        let query_params = vec![
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("grant_type", grant_type),
        ];

        let response = self.client.post(auth_url).query(&query_params).send().await?;
        let parsed_response = response.json::<AccessTokenResponse>().await?;

        self.bearer_token = BearerToken::new(
            parsed_response.access_token, 
            Utc::now().checked_add_signed(Duration::seconds(parsed_response.expires_in)).unwrap());

        Ok(())
    }

    pub async fn send_request<T: for<'a> Deserialize<'a> + Serialize + std::fmt::Debug + Into<Body>, U: for<'a> Deserialize<'a> + Serialize + std::fmt::Debug>(&mut self, path: String, method: Method, request: Option<T>) -> NetworkResult<U> {
        let url = format!("{}/{}", &self.base_url, path);

        if self.bearer_token.expiration < Utc::now() {
            self.refresh_bearer_token().await?;
        }

        let call = match request {
            Some(populated_request) => self.client.request(method.clone(), &url).bearer_auth(&self.bearer_token.token).header("Client-ID", &self.client_id).body(populated_request),
            None => self.client.request(method.clone(), &url).bearer_auth(&self.bearer_token.token).header("Client-ID", &self.client_id),
        };
        
        let response = call
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let parsed_response = response.json::<U>().await?;
                info!("IGDB game cover response body: {:?}", &parsed_response);
                Ok(parsed_response)
            },
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(NetworkError::new(NetworkErrorType::Unauthorized))
            },
            _ => {
                panic!("Unexpected response: {:?}", response.json().await?);
            }
        }
    }
}
