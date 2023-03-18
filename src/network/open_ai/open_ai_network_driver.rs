use serde::{Serialize, Deserialize};
use std::env;
use reqwest::Error;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    role: String,
    content: String,
}

impl ChatMessage {
    pub fn new(role: String, content: String) -> Self {
        Self {
            role,
            content,
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct ChatChoice {
    index: i32,
    message: ChatMessage,
    finish_reason: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct ChatUsage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

impl ChatRequest {
    fn new(existing_messages: &mut Vec<ChatMessage>) -> Self {
        let model = env::var("OPENAI_MODEL").unwrap();

        let mut messages = Vec::new();
        messages.push(ChatMessage::new(String::from("system"), env::var("OPENAI_SYSTEM_CONTENT")
            .expect("OpenAI System message not specified for the environment.")));
        messages.append(existing_messages);

        Self {
            model,
            messages,
        }
    }
}
    
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatResponse {
    id: String,
    object: String,
    created: i64,
    choices: Vec<ChatChoice>,
    usage: ChatUsage,
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

    pub async fn post_chat(&self, existing_messages: &mut Vec<ChatMessage>) -> Result<ChatResponse, Error> {
        let base_url = &self.base_url;
        let request = ChatRequest::new(existing_messages);
        
        let response = self.client.post(format!("{base_url}/chat/completions"))
            .bearer_auth(&self.bearer_token.token)
            .json(&request)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::CREATED => {
                let parsed_response = response.json::<ChatResponse>().await?;
                Ok(parsed_response)
            },
            reqwest::StatusCode::OK => {
                let parsed_response = response.json::<ChatResponse>().await?;
                Ok(parsed_response)
            },
            _ => {
                panic!("Unexpected response!");
            }
        }
    }
}
