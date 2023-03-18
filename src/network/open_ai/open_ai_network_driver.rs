use std::env;
use crate::errors::network_error::{
    NetworkError,
    NetworkErrorType,
};
use crate::network::open_ai::open_ai_models::{
    ChatMessage,
    ChatRequest,
    ChatResponse
};

type Result<T> = std::result::Result<T, NetworkError>;

struct BearerToken {
    token: String,
}

impl BearerToken {
    fn new(api_key: String) -> Self {
        Self {
            token: api_key.into(),
        }
    }
}

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
    pub fn new(base_url: &String) -> Self {
        Self {
            base_url: base_url.into(),
            bearer_token: BearerToken::new(env::var("OPENAI_API_KEY").unwrap()),
            client: create_client(),
        }
    }

    pub async fn post_chat(&self, existing_messages: Vec<ChatMessage>) -> Result<ChatResponse> {
        let base_url = &self.base_url;
        let request = ChatRequest::new(existing_messages);
        
        let response = self.client.post(format!("{base_url}/chat/completions"))
            .bearer_auth(&self.bearer_token.token)
            .json(&request)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let parsed_response = response.json::<ChatResponse>().await?;
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
