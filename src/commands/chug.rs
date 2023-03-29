use std::{env, time::Duration};
use itertools::Itertools;
use serenity::{
    client::Context,
    model::{channel::Message, prelude::ReactionType, user::User},
    framework::standard::{CommandResult, macros::command, Args},
    utils::Color, futures::StreamExt,
};
use crate::{services::{
    emoji_service::get_server_emoji_by_name,
    game_data_service::get_game_cover_url_by_name,
}, models::network_clients::GameNetworkClient};

#[command]
async fn chug(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut data = ctx.data.write().await;
    let client = data.get_mut::<GameNetworkClient>().unwrap();

    let bot_user = ctx.cache.current_user();
    let bot_avatar_url = match bot_user.avatar_url() {
        Some(avatar_url) => avatar_url,
        None => String::new(),
    };
    let chug_emote_name = env::var("CHUG_EMOTE_NAME").unwrap();
    let chug_emote = get_server_emoji_by_name(ctx, msg.guild_id, chug_emote_name).await.unwrap().unwrap();
    let game = args.single::<String>().unwrap();
    let game_cover_url = match get_game_cover_url_by_name(client, game.clone()).await.unwrap() {
        Some(url) => url,
        None => env::var("DEFAULT_GAME_IMAGE").unwrap(),
    };

    let mut chug_message = msg.channel_id.send_message(
        ctx, 
        |message| 
            message
                .add_embed(
                    |embed| 
                        embed
                            .author(|author| author.name(&bot_user.name).icon_url(&bot_avatar_url))
                            .title(format!("Chug Check started for \"{}\"", &game))
                            .description(format!(
                                "@{} has started a chug check for \"{}\". React with a {} if you're ready!", 
                                &msg.author, 
                                &game, 
                                chug_emote))
                            .color(Color::DARK_PURPLE)
                            .image(&game_cover_url)
                            .thumbnail(&game_cover_url)
                            .field("Game", &game, false)
                            .field("Chugsters", &msg.author, false)))
        .await?;

    let _ = chug_message.clone().react(ctx, chug_emote.clone()).await?;

    while let Some(reaction) = &chug_message
        .await_reactions(ctx)
        .timeout(Duration::from_secs(60))
        .message_id(chug_message.id)
        .build()
        .next()
        .await {
            if reaction.is_added() {
                let reacted_users = &reaction.as_inner_ref().users(ctx, chug_emote.clone(), None, None::<User>).await?;
                let users_string = reacted_users.clone().iter_mut().map(|user| &user.name).unique().join(", ");

                let _ = &chug_message.edit(
                    ctx, 
                    |message| 
                        message
                            .add_embed(
                                |embed| 
                                    embed
                                        .author(|author| author.name(&bot_user.name).icon_url(&bot_avatar_url))
                                        .title(format!("Chug Check started for \"{}\"", &game))
                                        .description(format!(
                                            "@{} has started a chug check for \"{}\". React with a {} if you're ready!", 
                                            &msg.author, 
                                            &game, 
                                            chug_emote))
                                        .color(Color::DARK_PURPLE)
                                        .image(&game_cover_url)
                                        .thumbnail(&game_cover_url)
                                        .field("Game", &game, false)
                                        .field("Chugsters", &users_string, false))
                );
            }
        };

    Ok(())
}
