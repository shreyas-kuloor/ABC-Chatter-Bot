use std::env;
use serenity::async_trait;
use serenity::model::prelude::Ready;
use serenity::model::channel::Message;
use serenity::prelude::*;
use models::active_threads::ActiveThreads;
use models::network_client::NetworkClient;
use network::open_ai::open_ai_network_driver::OpenAIClient;

mod commands;
mod network;
mod models;
mod services;
mod errors;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        commands::mention::on_mention(&ctx, &msg).await.unwrap();
        commands::reply_thread::on_reply_thread(&ctx, &msg).await.unwrap();
        commands::clear_threads::clear_inactive_threads(&ctx, &msg).await.unwrap();
    }
    
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected.", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("DISCORD_BOT_TOKEN")?;

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES 
        | GatewayIntents::MESSAGE_CONTENT; 

    let open_ai_client = OpenAIClient::new(&env::var("OPENAI_BASE_URL")?);

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await?;
    {
        let mut data = client.data.write().await;
        data.insert::<ActiveThreads>(Vec::default());
        data.insert::<NetworkClient>(open_ai_client);
    }

    client.start().await?;

    Ok(())
}
