use serenity::{
    client::Context,
    model::channel::Message,
    framework::standard::{CommandResult, macros::command, Args},
};

#[command]
async fn join(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let channel_to_connect = match channel_id {
        Some(channel) => channel,
        None => {
            let _ = msg.reply(ctx, "It doesn't look like you're in a voice channel. Please join a voice channel before running this command.").await?;

            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await.unwrap().clone();
    let _ = manager.join(guild_id, channel_to_connect).await;
    Ok(())
}