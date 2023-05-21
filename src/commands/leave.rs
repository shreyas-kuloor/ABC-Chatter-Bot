use log::warn;
use serenity::{
    client::Context,
    model::channel::Message,
    framework::standard::{CommandResult, macros::command, Args},
};

#[command]
async fn leave(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    
    let manager = songbird::get(ctx).await.unwrap().clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(err) = manager.remove(guild_id).await {
            warn!("Error occurred while trying to leave voice channel: {:?}", err);
            
        }
    }
    else {
        let _ = msg.reply(ctx, "It doesn't look like I'm in a voice channel, so this command won't do anything.").await?;
    }

    Ok(())
}