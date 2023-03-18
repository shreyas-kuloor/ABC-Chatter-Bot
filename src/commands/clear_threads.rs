use serenity::{
    prelude::*,
    model::{channel::Message, prelude::ChannelType},
};
use crate::models::active_threads::ActiveThreads;

pub async fn clear_inactive_threads(ctx: &Context, msg: &Message) -> Result<(), SerenityError> {
    let mut data = ctx.data.write().await;
    let active_threads = data.get_mut::<ActiveThreads>().unwrap();

    let channel_type = match msg.channel(ctx).await?.guild() {
        Some(guild) => guild.kind,
        None => ChannelType::Unknown,
    };

    if channel_type == ChannelType::Text {
        let archived_threads = msg.channel_id.get_archived_public_threads(ctx, None, None).await?.threads;

        for archived_thread in archived_threads {
            active_threads.retain(|t| t.0 != archived_thread.id.0);
        }
    }

    Ok(())
}