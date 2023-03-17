use std::env;
use serenity::async_trait;
use serenity::model::prelude::Ready;
use serenity::model::channel::Message;
use serenity::prelude::*;

mod commands;
mod network;
mod data;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(err) = commands::mention::on_mention(&ctx, &msg).await {
            println!("Mention reply failed: {:?}", err);
        };
    }
    
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected.", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_BOT_TOKEN").expect("No token in environment.");

    if let Err(err) = data::database_connection::connect_database().await {
        println!("Error connecting to database: {:?}", err);
    };

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
