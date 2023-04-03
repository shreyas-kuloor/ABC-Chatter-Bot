use std::{
    env,
    error::Error,
};
use rand::Rng;
use log::warn;
use serenity::{
    client::Context,
    model::channel::Message,
};

use crate::{
    models::network_clients::AINetworkClient, 
    services::{
        ai_chat_service::*,
        emoji_service::{get_server_emoji_names_string, get_server_emoji_by_name},
    }
};

pub async fn random_react_to_message(ctx: &Context, msg: &Message) -> Result<(), Box<dyn Error>> {
    let data = ctx.data.write().await;
    let open_ai_client = data.get::<AINetworkClient>().unwrap();

    if !msg.is_own(ctx) && msg.attachments.is_empty() && msg.embeds.is_empty() {
        let rand_react_upper_bound = env::var("RANDOM_REACT_UPPER_BOUND").unwrap().parse::<u64>().unwrap();
        let rand_num = rand::thread_rng().gen_range(0..rand_react_upper_bound);

        if rand_num == 0 {
            let emojis_string = get_server_emoji_names_string(ctx, msg.guild_id).await?;
            if emojis_string.is_some() {
                let bot_response = get_emoji_from_ai(open_ai_client, msg, emojis_string.unwrap()).await?;
                let matching_emoji = get_server_emoji_by_name(ctx, msg.guild_id, bot_response).await?;
                if let Some(emoji) = matching_emoji {
                    msg.react(ctx, emoji.clone()).await?;
                } else {
                    warn!("AI picked an invalid emoji, or the server has no emojis available.");
                }
            };
        }
    }
    
    Ok(())
}
