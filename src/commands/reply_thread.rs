use std::{
    env,
    error::Error,
};

use serenity::{
    client::Context,
    model::channel::Message,
};

use crate::{
    models::{
        active_threads::ActiveThreads, 
        network_client::NetworkClient
    }, 
    services::ai_chat_service::send_thread_to_ai
};

pub async fn on_reply_thread(ctx: &Context, msg: &Message) -> Result<(), Box<dyn Error>> {
    let data = ctx.data.write().await;
    let open_ai_client = data.get::<NetworkClient>().unwrap();
    let active_threads = data.get::<ActiveThreads>().unwrap();

    if active_threads.contains(&msg.channel_id) && !msg.is_own(ctx) {
        // let emojis = match msg.guild_id {
        //     Some(guild_id) => ctx.http().get_emojis(guild_id.0).await?,
        //     None => Vec::default(),
        // };

        // msg.react(ctx, emojis[0].clone()).await?;
        
        let message_limit = env::var("THREAD_MESSAGE_LIMIT").unwrap().parse::<u64>().unwrap();
        let mut thread_messages = msg
            .channel_id
            .messages(ctx, |m| m.limit(message_limit))
            .await?.clone();

        // ChannelId.messages returns messages from newest to oldest. We need to reverse
        // the order before sending it to AI, as it treats the last message as the prompt
        // and the previous messages as chat history.
        thread_messages.reverse();

        let typing = msg.channel_id.start_typing(&ctx.http)?;
        let bot_response = send_thread_to_ai(open_ai_client, ctx, thread_messages).await?;

        let _ = typing.stop();
        msg.channel_id.send_message(ctx, |m| m.content(bot_response)).await?;
    }
    
    Ok(())
}