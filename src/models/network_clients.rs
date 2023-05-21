use crate::{OpenAIClient, network::{games::igdb_network_driver::IGDBClient, stable_diffusion::stable_diffusion_network_driver::StableDiffusionClient, eleven_labs::eleven_labs_network_driver::ElevenLabsClient}};
use serenity::prelude::TypeMapKey;

pub struct AINetworkClient;

impl TypeMapKey for AINetworkClient {
    type Value = OpenAIClient;
}

pub struct GameNetworkClient;

impl TypeMapKey for GameNetworkClient {
    type Value = IGDBClient;
}

pub struct ImageGenNetworkClient;

impl TypeMapKey for ImageGenNetworkClient {
    type Value = StableDiffusionClient;
}

pub struct VoiceGenNetworkClient;

impl TypeMapKey for VoiceGenNetworkClient {
    type Value = ElevenLabsClient;
}