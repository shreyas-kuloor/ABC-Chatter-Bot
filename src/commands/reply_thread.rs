use serenity::{
    prelude::*,
    model::channel::Message, http::CacheHttp,
};
use crate::models::active_threads::ActiveThreads;

pub async fn on_reply_thread(ctx: &Context, msg: &Message) -> Result<(), SerenityError> {
    let mut data = ctx.data.write().await;
    let active_threads = data.get_mut::<ActiveThreads>().unwrap();

    if active_threads.contains(&msg.channel_id) && !msg.is_own(ctx) {
        let emojis = match msg.guild_id {
            Some(guild_id) => ctx.http().get_emojis(guild_id.0).await?,
            None => Vec::default(),
        };

        msg.react(ctx, emojis[0].clone()).await?;
    }
    
    Ok(())
}