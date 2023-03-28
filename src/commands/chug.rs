use serenity::{
    client::Context,
    model::channel::Message,
    framework::standard::{CommandResult, macros::command, Args},
};

#[command]
async fn chug(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let bot_user = ctx.cache.current_user();
    let bot_avatar_url = match bot_user.avatar_url() {
        Some(avatar_url) => avatar_url,
        None => String::new(),
    };
    let game = args.single::<String>().unwrap();

    msg.channel_id.send_message(
        ctx, 
        |message| 
            message
                .add_embed(
                    |embed| 
                        embed
                            .author(|author| author.name(bot_user.name).icon_url(bot_avatar_url))
                            .title(format!("Chug Check started for \"{}\"", game))))
        .await?;
    Ok(())
}