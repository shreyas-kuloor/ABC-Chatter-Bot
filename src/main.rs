mod effects;
mod commands;
mod network;
mod models;
mod services;
mod errors;
mod utils;

use dotenv::dotenv;
use std::env;
use effects::clear_threads::clear_inactive_threads;
use effects::leave_empty_channel::leave_empty_voice_channel;
use effects::mention::on_mention;
use effects::random_react::random_react_to_message;
use effects::reply_thread::on_reply_thread;
use log::info;
use network::eleven_labs::eleven_labs_network_driver::ElevenLabsClient;
use network::stable_diffusion::stable_diffusion_network_driver::StableDiffusionClient;
use serenity::async_trait;
use serenity::framework::StandardFramework;
use serenity::framework::standard::macros::group;
use serenity::model::prelude::Ready;
use serenity::model::channel::Message;
use serenity::model::voice::VoiceState;
use serenity::prelude::{EventHandler, Context, Client, GatewayIntents};
use models::active_threads::ActiveThreads;
use models::network_clients::{AINetworkClient, GameNetworkClient, ImageGenNetworkClient, VoiceGenNetworkClient};
use network::{open_ai::open_ai_network_driver::OpenAIClient, games::igdb_network_driver::IGDBClient};
use commands::*;
use songbird::SerenityInit;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        on_mention(&ctx, &msg).await.unwrap();
        on_reply_thread(&ctx, &msg).await.unwrap();
        clear_inactive_threads(&ctx, &msg).await.unwrap();
        random_react_to_message(&ctx, &msg).await.unwrap();
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, _new: VoiceState) {    
        leave_empty_voice_channel(&ctx, old).await.unwrap();
    }
    
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected.", ready.user.name);
    }
}

#[group]
#[commands(chug, image, voice, voices, join, leave, help)]
#[description = "Basic"]
struct General;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let token = env::var("DISCORD_BOT_TOKEN")?;

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES 
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::GUILD_VOICE_STATES; 

    let open_ai_client = OpenAIClient::new();
    let igdb_client = IGDBClient::new();
    let stable_diffusion_client = StableDiffusionClient::new();
    let eleven_labs_client = ElevenLabsClient::new();

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Warn)
        .level_for("abc_chatter_bot", log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;

    let framework = StandardFramework::new()
        .configure(|c| c
            .with_whitespace(true)
            .prefix(env::var("BOT_COMMAND_PREFIX").unwrap())
            .delimiters(vec![", ", ","])
            .case_insensitivity(true))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .type_map_insert::<ActiveThreads>(Vec::default())
        .type_map_insert::<AINetworkClient>(open_ai_client)
        .type_map_insert::<GameNetworkClient>(igdb_client)
        .type_map_insert::<ImageGenNetworkClient>(stable_diffusion_client)
        .type_map_insert::<VoiceGenNetworkClient>(eleven_labs_client)
        .framework(framework)
        .register_songbird() // Not immediately apparent, but this does a type_map_insert under the hood, so need to make sure the TypeMap does not get overwritten
        .await?;

    client.start().await?;

    Ok(())
}
