use serenity::{
    prelude::*,
    model::channel::Message,
};

pub async fn on_mention(ctx: &Context, msg: &Message) -> Result<(), SerenityError> {
    if msg.mentions_me(ctx).await? {
        msg.channel_id.create_public_thread(
            ctx, 
            msg.id, 
            |x| 
                x.name("Chatting :Chatting:")
                .auto_archive_duration(60)
                .rate_limit_per_user(0)
                .kind(serenity::model::prelude::ChannelType::PublicThread))
                .await?;
        Ok(())
    } else {
        Ok(())
    }
}