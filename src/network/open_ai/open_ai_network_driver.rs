use std::env;
use chrono::DateTime;
use log::info;

use crate::errors::network_error::{
    NetworkError,
    NetworkErrorType,
};
use crate::network::{
    models::network_models::{
        NetworkResult,
        BearerToken,
    },
    open_ai::open_ai_models::{
        ChatMessage,
        ChatRequest,
        ChatResponse
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

    pub async fn post_chat(&self, existing_messages: Vec<ChatMessage>) -> NetworkResult<ChatResponse> {
        let base_url = &self.base_url;
        let request = ChatRequest::new(existing_messages);
        
        info!("OpenAI request body: {:?}", &request);
        let response = self.client.post(format!("{base_url}/chat/completions"))
            .bearer_auth(&self.bearer_token)
            .json(&request)
            .send()
            .await?;

        info!("OpenAI response received: {:?}", &response);
        match response.status() {
            reqwest::StatusCode::OK => {
                let parsed_response = response.json::<ChatResponse>().await?;
                info!("OpenAI response body: {:?}", &parsed_response);
                Ok(parsed_response)
            },
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                Err(NetworkError::new(NetworkErrorType::TokenQuotaReached))
            },
            _ => {
                panic!("Unexpected response: {:?}", response.json().await?);
            }
        }
    }
}
