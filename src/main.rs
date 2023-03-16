use std::env;
use serenity::async_trait;
use serenity::model::prelude::Ready;
use serenity::model::channel::Message;
use serenity::prelude::*;

mod commands;
mod network;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        commands::mention::on_mention(&ctx, &msg).await;
    }
    
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected.", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_BOT_TOKEN").expect("No token in environment.");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES 
        | GatewayIntents::MESSAGE_CONTENT; 

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error during client creation.");

    if let Err(err) = client.start().await {
        println!("Client error: {:?}", err);
    }

}