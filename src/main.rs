mod commands;
mod network;
mod models;
mod services;
mod errors;
mod utils;

use std::env;
use log::info;
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
use models::network_clients::{AINetworkClient, GameNetworkClient, ImageGenNetworkClient};
use network::{
    open_ai::open_ai_network_driver::OpenAIClient, 
    games::igdb_network_driver::IGDBClient,
};
use commands::*;

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


#[group]
#[commands(chug)]
#[description = "Basic"]
struct General;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("DISCORD_BOT_TOKEN")?;

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES 
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS; 

    let open_ai_client = OpenAIClient::new();
    let igdb_client = IGDBClient::new();
    let stable_diffusion_client = StableDiffusionClient::new();

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
        .framework(framework)
        .await?;
    {
        let mut data = client.data.write().await;
        data.insert::<ActiveThreads>(Vec::default());
        data.insert::<AINetworkClient>(open_ai_client);
        data.insert::<GameNetworkClient>(igdb_client);
        data.insert::<ImageGenNetworkClient>(stable_diffusion_client);
    }

    client.start().await?;

    Ok(())
}
