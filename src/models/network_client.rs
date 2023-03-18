use crate::OpenAIClient;
use serenity::prelude::TypeMapKey;

pub struct NetworkClient;

impl TypeMapKey for NetworkClient {
    type Value = OpenAIClient;
}