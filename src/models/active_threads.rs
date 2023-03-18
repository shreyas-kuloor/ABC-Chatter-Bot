use serenity::{prelude::TypeMapKey, model::prelude::ChannelId};

pub struct ActiveThreads;

impl TypeMapKey for ActiveThreads {
    type Value = Vec<ChannelId>;
}