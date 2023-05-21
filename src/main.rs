mod commands;
mod network;
mod models;
mod services;
mod errors;
mod utils;

use std::env;
use log::{info, warn};
use network::eleven_labs::eleven_labs_network_driver::ElevenLabsClient;
use network::stable_diffusion::stable_diffusion_network_driver::StableDiffusionClient;
use serenity::async_trait;
use serenity::framework::StandardFramework;
use serenity::framework::standard::macros::group;
use serenity::model::prelude::Ready;
use serenity::model::channel::Message;
use serenity::prelude::{
    EventHandler,
    Context,
    Client,
    GatewayIntents
};
use models::active_threads::ActiveThreads;
use models::network_clients::{AINetworkClient, GameNetworkClient, ImageGenNetworkClient, VoiceGenNetworkClient};
use network::{
    open_ai::open_ai_network_driver::OpenAIClient, 
    games::igdb_network_driver::IGDBClient,
};
use commands::*;
use songbird::{SerenityInit, EventHandler as VoiceEventHandler, EventContext, Event};
use utils::voice_utils::TrackEndNotifier;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        commands::mention::on_mention(&ctx, &msg).await.unwrap();
        commands::reply_thread::on_reply_thread(&ctx, &msg).await.unwrap();
        commands::clear_threads::clear_inactive_threads(&ctx, &msg).await.unwrap();
        commands::random_react::random_react_to_message(&ctx, &msg).await.unwrap();
    }
    
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected.", ready.user.name);
    }
}

#[async_trait]
impl VoiceEventHandler for TrackEndNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(_track_list) = ctx {
            info!("Track ended");

            let manager = songbird::get(&self.ctx).await?.clone();
            let has_handler = manager.get(self.guild_id).is_some();

            if has_handler {
                if let Err(err) = manager.remove(self.guild_id).await {
                    warn!("Error occurred while trying to leave voice channel: {:?}", err);
                    
                }
                
                info!("Left voice channel");
            }
            else {
                warn!("Track ended while not in a voice channel.");
            }
        }

        None
    }
}

#[group]
#[commands(chug, image, help, voice, voices)]
#[description = "Basic"]
struct General;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        .level_for("songbird", log::LevelFilter::Trace)
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
        .register_songbird()
        .await?;

    client.start().await?;

    Ok(())
}
