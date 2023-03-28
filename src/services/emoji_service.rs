use std::error::Error;
use itertools::Itertools;
use serenity::{
    model::prelude::{GuildId, Emoji},
    client::Context
};
use log::warn;

pub async fn get_server_emoji_by_name(ctx: &Context, guild_id: Option<GuildId>, emoji_name: String) -> Result<Option<Emoji>, Box<dyn Error>> {
    if let Some(guild_id) = guild_id {
        let mut emojis = guild_id.emojis(&ctx.http).await?;
        let matching_emoji = emojis.iter_mut().find(|e| e.name == emoji_name);

        Ok(matching_emoji.cloned())
    } else {
        warn!("Guild ID not found.");
        Ok(None)
    }
}

pub async fn get_server_emoji_names_string(ctx: &Context, guild_id: Option<GuildId>) -> Result<Option<String>, Box<dyn Error>> {
    if let Some(guild_id) = guild_id {
        let mut emojis = guild_id.emojis(&ctx.http).await?;
        let emojis_string = emojis.iter_mut().map(|e| e.name.clone()).join(", ");

        Ok(Some(emojis_string))
    } else {
        warn!("Guild ID not found.");
        Ok(None)
    }
}
