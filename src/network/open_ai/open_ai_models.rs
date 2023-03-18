use std::env;
use serde::{
    Serialize, 
    Deserialize
};

pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn new(role: Role, content: String) -> Self {
        Self {
            role: match role {
                Role::System => String::from("system"),
                Role::User => String::from("user"),
                Role::Assistant => String::from("assistant"),
            },
            content,
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ChatChoice {
    index: i32,
    pub message: ChatMessage,
    finish_reason: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct ChatUsage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

impl ChatRequest {
    pub fn new(existing_messages: Vec<ChatMessage>) -> Self {
        let model = env::var("OPENAI_MODEL").unwrap();

        let mut messages = Vec::new();
        messages.push(ChatMessage::new(Role::System, env::var("OPENAI_SYSTEM_CONTENT").unwrap()));
        messages.extend(existing_messages);

        Self {
            model,
            messages,
        }
    }
}
    
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub choices: Vec<ChatChoice>,
    usage: ChatUsage,
}