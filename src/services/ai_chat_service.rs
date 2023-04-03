use std::error::Error;

use reqwest::Method;
use serenity::{
    model::prelude::Message, 
    prelude::Context
};

use crate::{
    network::open_ai::{
        open_ai_network_driver::OpenAIClient,
        open_ai_models::{
            ChatMessage, 
            Role, ChatRequest, ChatResponse
        }
    },
    errors::network_error::NetworkErrorType,
};

pub async fn send_thread_to_ai(client: &OpenAIClient, ctx: &Context, messages: Vec<Message>) -> Result<String, Box<dyn Error>> {
    let chat_messages: Vec<ChatMessage> = messages.clone().iter_mut().map(|m| {
        let is_bot = m.is_own(ctx);
        let role = if is_bot { Role::Assistant } else { Role::User };
        ChatMessage::new(role, m.content.clone().to_string())
    }).collect();

    let ai_request = ChatRequest::new(chat_messages);

    let message_content = match client.send_request::<ChatRequest, ChatResponse>("chat/completions".to_string(), Method::POST, Some(ai_request)).await {
        Ok(resp) => resp.choices[0].message.content.clone(),
        Err(err) => {
            match err.error_type {
                NetworkErrorType::TokenQuotaReached => String::from("Sorry! I've reached my limit for this month. Please ask the administrator to check their OpenAI billing details."),
                NetworkErrorType::Unknown => String::from("Sorry! An unknown error occurred. Please contact the administrator for details."),
                _ => String::from("Sorry! An unknown error occurred. Please contact the administrator for details."),
            }
        },
    };

    Ok(message_content)
}


pub async fn get_emoji_from_ai(client: &OpenAIClient, message: &Message, emoji_list: String) -> Result<String, Box<dyn Error>> {
    let mut messages = vec![message];
    let chat_messages: Vec<ChatMessage> = messages.iter_mut().map(|m| {
        let prompt = format!("Pick one discord emote from ({}) that best fits the message '{}'. You must respond with only a single word from the list.", emoji_list, m.content);
        ChatMessage::new(Role::User, prompt)
    }).collect();

    let ai_request = ChatRequest::new(chat_messages);

    let message_content = match client.send_request::<ChatRequest, ChatResponse>("chat/completions".to_string(), Method::POST, Some(ai_request)).await {
        Ok(resp) => resp.choices[0].message.content.clone(),
        Err(err) => {
            match err.error_type {
                NetworkErrorType::TokenQuotaReached => String::from("Sorry! I've reached my limit for this month. Please ask the administrator to check their OpenAI billing details."),
                NetworkErrorType::Unknown => String::from("Sorry! An unknown error occurred. Please contact the administrator for details."),
                _ => String::from("Sorry! An unknown error occurred. Please contact the administrator for details."),
            }
        },
    };

    Ok(message_content.replace('.', ""))
}
