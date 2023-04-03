use std::error::Error;

use serenity::{
    prelude::*,
    model::channel::Message,
};

use crate::{
    models::{
        active_threads::ActiveThreads, 
        network_clients::AINetworkClient,
    },
    services::ai_chat_service::send_thread_to_ai
};

pub async fn on_mention(ctx: &Context, msg: &Message) -> Result<(), Box<dyn Error>> {
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

        active_threads.push(channel.id);
        drop(data);

        let data = ctx.data.write().await;
        let client = data.get::<AINetworkClient>().unwrap();

        let thread_messages = vec![msg.clone()];
        let typing = msg.channel_id.start_typing(&ctx.http)?;
        
        let bot_response = send_thread_to_ai(client, ctx, thread_messages).await?;

        let _ = typing.stop();
        
        channel
            .send_message(
                ctx, |m| m.content(bot_response)).await?;
    }
    
    Ok(())
}
