use serenity::{
    prelude::*,
    model::channel::Message,
};

pub async fn on_mention(ctx: &Context, msg: &Message) {
    let bot_user_id = ctx.cache.current_user_id();
    if msg.mentions.iter().any(|user| user.id == bot_user_id) {
        if let Err(err) = msg.reply_ping(ctx, "Hello!").await {
            println!("Error replying to mention: {:?}", err);
        };
    }
}