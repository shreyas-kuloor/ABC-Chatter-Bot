use std::{env, time::Duration};
use log::{info, warn};
use itertools::Itertools;
use serenity::{
    client::Context,
    model::{channel::Message, user::User, prelude::{EventType, ReactionType, UserId}, event::Event},
    framework::standard::{CommandResult, macros::command, Args},
    utils::Color, futures::StreamExt, collector::EventCollectorBuilder, builder::CreateEmbed,
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

        let mut default_embed = CreateEmbed::default();
        let embed_template = default_embed
            .author(|author| author.name(&bot_user.name).icon_url(&bot_avatar_url))
            .title(format!("Chug Check started for \"{}\"", &game))
            .description(format!(
                "{} has started a chug check for \"{}\". React with a {} if you're ready!

                The creator of the chug check may react with 📢 to ping all current chugsters.

                The creator of the chug check may react with ❌ to cancel.", 
                &msg.author, 
                &game, 
                chug_emote))
            .color(Color::DARK_PURPLE)
            .thumbnail(&game_cover_url)
            .field("Game", &game, false);

        let mut chug_message = msg.channel_id.send_message(
            ctx, 
            |message| 
                message
                    .set_embed(embed_template.clone().field("Chugsters", "", false).clone()))
            .await?;

        let _ = &chug_message.react(ctx, chug_emote.clone()).await?;
        let _ = &chug_message.react(ctx, ReactionType::Unicode("📢".to_string())).await?;
        let _ = &chug_message.react(ctx, ReactionType::Unicode("❌".to_string())).await?;

        let ctx = ctx.clone();
        let msg = msg.clone();
        let embed_template = embed_template.clone();

        tokio::spawn(async move {
            let thread_ctx = &ctx.clone();
            let original_message = msg.clone();
            let thread_embed = embed_template.clone();
            let mut collector = EventCollectorBuilder::new(ctx)
                .add_event_type(EventType::ReactionAdd)
                .add_event_type(EventType::ReactionRemove)
                .add_message_id(chug_message.id)
                .timeout(Duration::from_secs(chug_timeout))
                .build().unwrap();

            while let Some(event) = collector
                .next()
                .await {
                    match event.as_ref() {
                        Event::ReactionAdd(addition) => {
                            let addition_author_id = addition.reaction.user(thread_ctx).await.unwrap().id;
                            let message_author_id = original_message.author.id;

                            if addition.reaction.emoji == ReactionType::Unicode("❌".to_string()) && addition_author_id == message_author_id {
                                let _ = &chug_message.edit(
                                    thread_ctx, 
                                    |message| 
                                        message
                                            .set_embed(thread_embed.clone().field(format!("❌ Chug cancelled by {}", &original_message.author.name), "", false).clone()))
                                    .await.unwrap();

                                break;
                            } 
                            else if addition.reaction.emoji == ReactionType::Unicode("📢".to_string()) && addition_author_id == message_author_id {
                                let reacted_users = addition.reaction.users(thread_ctx, chug_emote.clone(), None, None::<User>).await.unwrap();
                                let users_string = reacted_users.clone().iter().filter(|user| !user.bot).unique().join(", ");

                                if !users_string.is_empty() {
                                    let _ = &chug_message.reply(thread_ctx, format!("CHUG ALERT 📢: {}", users_string)).await.unwrap();
                                }
                            } 
                            else {
                                let reacted_users = addition.reaction.users(thread_ctx, chug_emote.clone(), None, None::<User>).await.unwrap();
                                let users_string = reacted_users.clone().iter().filter(|user| !user.bot).unique().join(", ");
                
                                let _ = &chug_message.edit(
                                    thread_ctx,
                                    |message| 
                                        message
                                            .set_embed(thread_embed.clone().field("Chugsters", &users_string, false).clone()))
                                    .await.unwrap();
                            }
                        },
                        Event::ReactionRemove(removal) => {
                            let reacted_users = removal.reaction.users(thread_ctx, chug_emote.clone(), None, None::<User>).await.unwrap();
                            let users_string = reacted_users.clone().iter().filter(|user| !user.bot).unique().join(", ");
            
                            let _ = &chug_message.edit(
                                thread_ctx, 
                                |message| 
                                    message
                                        .set_embed(thread_embed.clone().field("Chugsters", &users_string, false).clone()))
                                .await.unwrap();
                        },
                        _ => warn!("Some other event occurred even though it was not configured"),
                    }
                }
        });

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

        let mut default_embed = CreateEmbed::default();
        let embed_template = default_embed
            .author(|author| author.name(&bot_user.name).icon_url(&bot_avatar_url))
            .title(format!("Chug Check started for multiple games! {}", chug_emote))
            .description(format!(
                "{} has started a chug check for multiple games. React with the emoji next to the game(s) you want to play!

                The creator of the chug check may react with 📢 to ping all current chugsters.

                If you started the chug check, you may react with ❌ to cancel.", 
                &msg.author))
            .color(Color::DARK_PURPLE)
            .thumbnail(env::var("CHUG_POLL_IMAGE").unwrap());

        let mut chug_message = msg.channel_id.send_message(
            ctx, 
            |message| 
                message
                    .set_embed(embed_template.clone().field("Games and Chugsters", &options, false).clone()))
            .await?;

        for i in 1..=args.len() {
            let _ = &chug_message.react(ctx, ReactionType::Unicode(get_unicode_from_number(i).unwrap())).await?;
        }
        let _ = &chug_message.react(ctx, ReactionType::Unicode("📢".to_string())).await?;
        let _ = &chug_message.react(ctx, ReactionType::Unicode("❌".to_string())).await?;

        let ctx = ctx.clone();
        let msg = msg.clone();
        let embed_template = embed_template.clone();

        tokio::spawn(async move {
            let thread_ctx = &ctx.clone();
            let original_message = msg.clone();
            let thread_embed = embed_template.clone();
            let mut collector = EventCollectorBuilder::new(thread_ctx)
                .add_event_type(EventType::ReactionAdd)
                .add_event_type(EventType::ReactionRemove)
                .add_message_id(chug_message.id)
                .timeout(Duration::from_secs(chug_timeout))
                .build().unwrap();

            let mut cancelled = false;

            while let Some(event) = collector
                .next()
                .await {
                    match event.as_ref() {
                        Event::ReactionAdd(addition) => {
                            let mut updated_fields: Vec<String> = Vec::new();
                            for i in 1..=args.len() {
                                let reacted_users = addition
                                    .reaction
                                    .users(thread_ctx, ReactionType::Unicode(get_unicode_from_number(i).unwrap()), None, None::<User>)
                                    .await
                                    .unwrap();
                                let users_string = reacted_users.clone().iter().filter(|user| !user.bot).unique().join(", ");

                                if users_string.is_empty() {
                                    updated_fields.push(format!("{} {}", get_unicode_from_number(i).unwrap(), games[i-1]));
                                } else {
                                    updated_fields.push(format!("{} {} - {}", get_unicode_from_number(i).unwrap(), games[i-1], users_string));
                                }
                            }

                            let addition_author_id = addition.reaction.user(thread_ctx).await.unwrap().id;
                            let message_author_id = original_message.author.id;

                            if addition.reaction.emoji == ReactionType::Unicode("❌".to_string()) && addition_author_id == message_author_id {
                                let _ = &chug_message.edit(
                                    thread_ctx, 
                                    |message| 
                                        message
                                            .set_embed(thread_embed
                                                .clone()
                                                .field("Games and Chugsters", &updated_fields.join("\n"), false)
                                                .field(format!("❌ Chug cancelled by {}", &original_message.author.name), "", false)
                                                .clone())
                                ).await.unwrap();
    
                                cancelled = true;
                                break;
                            }
                            else if addition.reaction.emoji == ReactionType::Unicode("📢".to_string()) && addition_author_id == message_author_id {
                                let mut total_reacted_users: Vec<User> = Vec::new();
                                for i in 1..=args.len() {
                                    let mut reacted_users = addition
                                        .reaction
                                        .users(thread_ctx, ReactionType::Unicode(get_unicode_from_number(i).unwrap()), None, None::<User>)
                                        .await
                                        .unwrap();
                                    total_reacted_users.append(&mut reacted_users);
                                }
                                let users_string = total_reacted_users.clone().iter().filter(|user| !user.bot).unique().join(", ");
                                if !users_string.is_empty() {
                                    let _ = &chug_message.reply(thread_ctx, format!("CHUG ALERT 📢: {}", users_string)).await.unwrap();
                                }
                            }  
                            else {
                                let _ = &chug_message.edit(
                                    thread_ctx, 
                                    |message| 
                                        message
                                            .set_embed(thread_embed.clone().field("Games and Chugsters", &updated_fields.join("\n"), false).clone())
                                ).await.unwrap();
                            }
                        },
                        Event::ReactionRemove(removal) => {
                            let mut updated_fields: Vec<String> = Vec::new();
                            for i in 1..=args.len() {
                                let reacted_users = removal
                                    .reaction
                                    .users(thread_ctx, ReactionType::Unicode(get_unicode_from_number(i).unwrap()), None, None::<User>)
                                    .await.unwrap();
                                let users_string = reacted_users.clone().iter().filter(|user| !user.bot).unique().join(", ");

                                if users_string.is_empty() {
                                    updated_fields.push(format!("{} {}", get_unicode_from_number(i).unwrap(), games[i-1]));
                                } else {
                                    updated_fields.push(format!("{} {} - {}", get_unicode_from_number(i).unwrap(), games[i-1], users_string));
                                }
                            }

                            let _ = &chug_message.edit(
                                thread_ctx, 
                                |message| 
                                    message
                                        .set_embed(thread_embed.clone().field("Games and Chugsters", &updated_fields.join("\n"), false).clone())
                            ).await.unwrap();
                        },
                        _ => warn!("Some other event occurred even though it was not configured"),
                    }
                }
            
            if !cancelled {
                let mut total_reacted_users: Vec<User> = Vec::new();
                for i in 1..=args.len() {
                    let mut reacted_users = chug_message
                        .reaction_users(thread_ctx, ReactionType::Unicode(get_unicode_from_number(i).unwrap()), None, None::<UserId>)
                        .await
                        .unwrap();
                    total_reacted_users.append(&mut reacted_users);
                }
                let users_string = total_reacted_users.clone().iter().filter(|user| !user.bot).unique().join(", ");
                if !users_string.is_empty() {
                    let _ = &chug_message.reply(thread_ctx, format!("CHUG ALERT 📢: {} \nFALSE CHUGGERS WILL BE PROSECUTED", users_string)).await.unwrap();
                }
            }
        });
    }

    Ok(())
}
