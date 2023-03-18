use serenity::{
    prelude::*,
    model::channel::Message,
};
use crate::models::active_threads::ActiveThreads;

pub async fn on_mention(ctx: &Context, msg: &Message) -> Result<(), SerenityError> {
    let mut data = ctx.data.write().await;
    let active_threads = data.get_mut::<ActiveThreads>().unwrap();
    
    if msg.mentions_me(ctx).await? && !active_threads.contains(&msg.channel_id) {
        let channel = msg.channel_id.create_public_thread(
            ctx, 
            msg.id, 
            |x| 
                x.name("Chatting")
                .auto_archive_duration(60)
                .rate_limit_per_user(0)
                .kind(serenity::model::prelude::ChannelType::PublicThread))
                .await?;
        
        channel
            .send_message(
                ctx, |m| m.content("Hello! How can I help?")).await?;

        active_threads.push(channel.id);
    }
    
    Ok(())
}