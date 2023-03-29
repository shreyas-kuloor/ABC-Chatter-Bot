use crate::{OpenAIClient, network::games::igdb_network_driver::IGDBClient};
use serenity::prelude::TypeMapKey;

pub struct AINetworkClient;

impl TypeMapKey for AINetworkClient {
    type Value = OpenAIClient;
}

pub struct GameNetworkClient;

impl TypeMapKey for GameNetworkClient {
    type Value = IGDBClient;
}
