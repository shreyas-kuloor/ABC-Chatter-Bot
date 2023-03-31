use std::{env, time::Duration};
use log::info;
use itertools::Itertools;
use serenity::{
    client::Context,
    model::{channel::Message, user::User, prelude::{EventType, ReactionType}, event::Event},
    framework::standard::{CommandResult, macros::command, Args},
    utils::Color, futures::StreamExt, collector::EventCollectorBuilder,
};
use crate::{services::{
    emoji_service::get_server_emoji_by_name,
    game_data_service::get_game_cover_url_by_name,
}, models::network_clients::GameNetworkClient, utils::number_utils::get_unicode_from_number};

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
    let chug_timeout = env::var("CHUG_TIMEOUT_SECONDS").unwrap().parse::<u64>().unwrap();

    if args.len() == 1 {
        let game = args.single::<String>().unwrap();

        let game_cover_url = match get_game_cover_url_by_name(client, game.clone()).await.unwrap() {
            Some(url) => url,
            None => env::var("DEFAULT_GAME_IMAGE").unwrap(),
        };

        msg.delete(ctx).await?;

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
                                    "{} has started a chug check for \"{}\". React with a {} if you're ready!", 
                                    &msg.author, 
                                    &game, 
                                    chug_emote))
                                .color(Color::DARK_PURPLE)
                                .thumbnail(&game_cover_url)
                                .field("Game", &game, false)
                                .field("Chugsters", "", false)))
            .await?;

        let _ = chug_message.clone().react(ctx, chug_emote.clone()).await?;

        let mut collector = EventCollectorBuilder::new(ctx)
            .add_event_type(EventType::ReactionAdd)
            .add_event_type(EventType::ReactionRemove)
            .add_message_id(chug_message.id)
            .timeout(Duration::from_secs(chug_timeout))
            .build()?;

        while let Some(event) = collector
            .next()
            .await {
                match event.as_ref() {
                    Event::ReactionAdd(reaction) => {
                        let reacted_users = reaction.reaction.users(ctx, chug_emote.clone(), None, None::<User>).await?;
                        let users_string = reacted_users.clone().iter().filter(|user| !user.bot).unique().join(", ");
        
                        let _ = &chug_message.edit(
                            ctx, 
                            |message| 
                                message
                                    .embed(
                                        |embed| 
                                            embed
                                                .author(|author| author.name(&bot_user.name).icon_url(&bot_avatar_url))
                                                .title(format!("Chug Check started for \"{}\"", &game))
                                                .description(format!(
                                                    "{} has started a chug check for \"{}\". React with a {} if you're ready!", 
                                                    &msg.author, 
                                                    &game, 
                                                    chug_emote))
                                                .color(Color::DARK_PURPLE)
                                                .thumbnail(&game_cover_url)
                                                .field("Game", &game, false)
                                                .field("Chugsters", &users_string, false))
                        ).await?;
                    },
                    Event::ReactionRemove(removal) => {
                        let reacted_users = removal.reaction.users(ctx, chug_emote.clone(), None, None::<User>).await?;
                        let users_string = reacted_users.clone().iter().filter(|user| !user.bot).unique().join(", ");
        
                        let _ = &chug_message.edit(
                            ctx, 
                            |message| 
                                message
                                    .embed(
                                        |embed| 
                                            embed
                                                .author(|author| author.name(&bot_user.name).icon_url(&bot_avatar_url))
                                                .title(format!("Chug Check started for \"{}\"", &game))
                                                .description(format!(
                                                    "{} has started a chug check for \"{}\". React with a {} if you're ready!", 
                                                    &msg.author, 
                                                    &game, 
                                                    chug_emote))
                                                .color(Color::DARK_PURPLE)
                                                .thumbnail(&game_cover_url)
                                                .field("Game", &game, false)
                                                .field("Chugsters", &users_string, false))
                        ).await?;
                    }
                    _ => info!("Some other event occurred"),
                }
            };

        info!("Exited loop");
    }
    else if args.len() > 1 && args.len() < 10 {
        let mut games: Vec<String> = Vec::new();
        for arg in args.iter::<String>() {
            games.push(arg.unwrap_or_default());
        }

        msg.delete(ctx).await?;

        let options = games
            .iter_mut()
            .enumerate()
            .map(|(i, game)| format!("{} {}", get_unicode_from_number(i+1).unwrap(), game))
            .join("\n");

        let embed_fields = games.iter().map(|game| (format!("{} Chugsters", game), "", true));

        let mut chug_message = msg.channel_id.send_message(
            ctx, 
            |message| 
                message
                    .add_embed(
                        |embed| 
                            embed
                                .author(|author| author.name(&bot_user.name).icon_url(&bot_avatar_url))
                                .title("Chug Check started for multiple games!")
                                .description(format!(
                                    "{} has started a chug check for multiple games. React with the emoji next to the game you want to play! {}", 
                                    &msg.author,
                                    chug_emote))
                                .color(Color::DARK_PURPLE)
                                .thumbnail(env::var("CHUG_POLL_IMAGE").unwrap())
                                .field("Games", &options, false)
                                .fields(embed_fields)))
            .await?;

        for i in 1..=args.len() {
            let _ = &chug_message.react(ctx, ReactionType::Unicode(get_unicode_from_number(i).unwrap())).await?;
        }

        let mut collector = EventCollectorBuilder::new(ctx)
            .add_event_type(EventType::ReactionAdd)
            .add_event_type(EventType::ReactionRemove)
            .add_message_id(chug_message.id)
            .timeout(Duration::from_secs(chug_timeout))
            .build()?;

        while let Some(event) = collector
            .next()
            .await {
                match event.as_ref() {
                    Event::ReactionAdd(reaction) => {
                        let mut updated_fields: Vec<(String, String, bool)> = Vec::new();
                        for i in 1..=args.len() {
                            let reacted_users = reaction.reaction.users(ctx, ReactionType::Unicode(get_unicode_from_number(i).unwrap()), None, None::<User>).await?;
                            let users_string = reacted_users.clone().iter().filter(|user| !user.bot).unique().join(", ");
                            updated_fields.push((format!("{} Chugsters", games[i-1]), users_string, true));
                        }
        
                        let _ = &chug_message.edit(
                            ctx, 
                            |message| 
                                message
                                    .embed(
                                        |embed| 
                                            embed
                                                .author(|author| author.name(&bot_user.name).icon_url(&bot_avatar_url))
                                                .title("Chug Check started for multiple games!")
                                                .description(format!(
                                                    "{} has started a chug check for multiple games. React with the emoji next to the game you want to play! {}", 
                                                    &msg.author,
                                                    chug_emote))
                                                .color(Color::DARK_PURPLE)
                                                .thumbnail(env::var("CHUG_POLL_IMAGE").unwrap())
                                                .field("Games", &options, false)
                                                .fields(updated_fields))
                        ).await?;
                    },
                    Event::ReactionRemove(removal) => {
                        let mut updated_fields: Vec<(String, String, bool)> = Vec::new();
                        for i in 1..=args.len() {
                            let reacted_users = removal.reaction.users(ctx, ReactionType::Unicode(get_unicode_from_number(i).unwrap()), None, None::<User>).await?;
                            let users_string = reacted_users.clone().iter().filter(|user| !user.bot).unique().join(", ");
                            updated_fields.push((format!("{} Chugsters", games[i-1]), users_string, true));
                        }
        
                        let _ = &chug_message.edit(
                            ctx, 
                            |message| 
                                message
                                    .embed(
                                        |embed| 
                                            embed
                                                .author(|author| author.name(&bot_user.name).icon_url(&bot_avatar_url))
                                                .title("Chug Check started for multiple games!")
                                                .description(format!(
                                                    "{} has started a chug check for multiple games. React with the emoji next to the game you want to play! {}", 
                                                    &msg.author,
                                                    chug_emote))
                                                .color(Color::DARK_PURPLE)
                                                .thumbnail(env::var("CHUG_POLL_IMAGE").unwrap())
                                                .field("Games", &options, false)
                                                .fields(updated_fields))
                        ).await?;
                    }
                    _ => info!("Some other event occurred"),
                }
            };
    }

    Ok(())
}
