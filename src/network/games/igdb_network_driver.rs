use std::{env, vec};
use chrono::{Utc, Duration, DateTime};
use log::info;

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

use super::igdb_models::GameResponse;

pub struct IGDBClient {
    base_url: String,
    bearer_token: BearerToken,
    client: reqwest::Client,
}

fn create_client() -> reqwest::Client {
    let client = reqwest::Client::new();
    client
}

impl IGDBClient {
    pub fn new() -> Self {
        let mut igdb_client = Self {
            base_url: env::var("IGDB_BASE_URL").unwrap(),
            bearer_token: BearerToken::new(String::default(), DateTime::default()),
            client: create_client(),
        };

        igdb_client.refresh_bearer_token();

        igdb_client
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

    pub async fn post_game_cover_details(&mut self, game_name: String) -> NetworkResult<GameResponse> {
        let base_url = self.base_url.clone();
        let request = format!("fields cover.*; search \"{}\"", game_name);

        if self.bearer_token.expiration < Utc::now() {
            self.refresh_bearer_token();
        }
        
        info!("IGDB game cover request body: {}", &request);
        let response = self.client.post(format!("{base_url}/games"))
            .bearer_auth(&self.bearer_token.token)
            .body(request)
            .send()
            .await?;

        info!("IGDB game cover response received: {:?}", &response);
        match response.status() {
            reqwest::StatusCode::OK => {
                let parsed_response = response.json::<GameResponse>().await?;
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
