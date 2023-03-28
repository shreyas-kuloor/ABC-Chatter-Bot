use std::env;
use serenity::{
    client::Context,
    model::channel::Message,
    framework::standard::{CommandResult, macros::command, Args},
    utils::Color,
};
use crate::services::emoji_service::get_server_emoji_by_name;

#[command]
async fn chug(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let bot_user = ctx.cache.current_user();
    let bot_avatar_url = match bot_user.avatar_url() {
        Some(avatar_url) => avatar_url,
        None => String::new(),
    };
    let chug_emote_name = env::var("CHUG_EMOTE_NAME").unwrap();
    let chug_emote = get_server_emoji_by_name(ctx, msg.guild_id, chug_emote_name).await.unwrap().unwrap();
    let game = args.single::<String>().unwrap();

    let chug_message = msg.channel_id.send_message(
        ctx, 
        |message| 
            message
                .add_embed(
                    |embed| 
                        embed
                            .author(|author| author.name(bot_user.name).icon_url(bot_avatar_url))
                            .title(format!("Chug Check started for \"{}\"", game))
                            .description(format!(
                                "@{} has started a chug check for \"{}\". React with a {} if you're ready!", 
                                &msg.author, 
                                game, 
                                chug_emote))
                            .color(Color::DARK_PURPLE)
                            .field("Game", game, false)
                            .field("Chugsters", &msg.author, false)))
        .await?;
    Ok(())
}
