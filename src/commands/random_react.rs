use std::env;
use rand::Rng;
use itertools::Itertools;
use log::warn;
use serenity::{
    client::Context,
    model::channel::Message,
    prelude::SerenityError,
};

use crate::{
    models::network_client::NetworkClient, 
    services::ai_chat_service::*
};

pub async fn random_react_to_message(ctx: &Context, msg: &Message) -> Result<(), SerenityError> {
    let data = ctx.data.write().await;
    let open_ai_client = data.get::<NetworkClient>().unwrap();

    if !msg.is_own(ctx) && msg.attachments.is_empty() && msg.embeds.is_empty() {
        let rand_react_upper_bound = env::var("RANDOM_REACT_UPPER_BOUND").unwrap().parse::<u64>().unwrap();
        let rand_num = rand::thread_rng().gen_range(0..rand_react_upper_bound);

        if rand_num == 0 {
            if let Some(guild_id) = msg.guild_id {
                let emojis = guild_id.emojis(&ctx.http).await?;

                let emojis_string = emojis.clone().iter_mut().map(|e| e.name.clone()).join(", ");
                let bot_response = get_emoji_from_ai(open_ai_client, msg, emojis_string).await;

                let mut emojis_clone = emojis.clone();
                let matching_emoji = emojis_clone.iter_mut().find(|e| e.name == bot_response);
                if let Some(emoji) = matching_emoji {
                    msg.react(ctx, emoji.clone()).await?;
                } else {
                    warn!("AI picked an invalid emoji, or the server has no emojis available.");
                }
            } else {
                warn!("Guild ID not found.");
            }
        }
    }
    
    Ok(())
}