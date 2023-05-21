use std::env;
use serenity::{prelude::Context, model::prelude::Message, framework::standard::{Args, CommandResult, macros::command}, utils::Color};

#[command]
#[sub_commands(mention, chug, image, voice_help, voices_help, join_help, leave_help)]
pub async fn help(
    ctx: &Context, 
    msg: &Message, 
    _args: Args, 
) -> CommandResult {
    let bot_user = ctx.cache.current_user();
    let bot_avatar_url = match bot_user.avatar_url() {
        Some(avatar_url) => avatar_url,
        None => String::new(),
    };

    let _ = msg.channel_id.send_message(
        ctx, 
        |message| 
            message
                .reference_message(msg)
                .embed(|embed| embed
                    .author(|author| author.name(&bot_user.name).icon_url(&bot_avatar_url))
                    .title("Help")
                    .description("To get help with an individual command, pass its name as an argument to this command (/help `{command}`).")
                    .color(Color::TEAL)
                    .field("Commands", "`mention`, `chug`, `image`, `voice`, `voices`, `join`, `leave`", false)))
        .await?;
    Ok(())
}

#[command]
async fn mention(
    ctx: &Context, 
    msg: &Message, 
    _args: Args, 
) -> CommandResult {
    let bot_user = ctx.cache.current_user();
    let bot_avatar_url = match bot_user.avatar_url() {
        Some(avatar_url) => avatar_url,
        None => String::new(),
    };

    let _ = msg.channel_id.send_message(
        ctx, 
        |message| 
            message
                .reference_message(msg)
                .embed(|embed| embed
                    .author(|author| author.name(&bot_user.name).icon_url(&bot_avatar_url))
                    .title("Command: Mention")
                    .description(
                        "The mention command is used to communicate with me directly. You can ask me questions, or just have a normal conversation!
                        Once the conversation has been kicked off, you can continue the conversation with me in the thread I create.
                        The thread will automatically archive after 1 hour.")
                    .color(Color::TEAL)
                    .field("Usage", format!("@{} `{{message}}`", bot_user.name), false)
                    .field("Arguments", "`message`: The message to prompt the beginning of a conversation.", false)))
        .await?;
    Ok(())
}

#[command]
async fn chug(
    ctx: &Context, 
    msg: &Message, 
    _args: Args, 
) -> CommandResult {
    let bot_user = ctx.cache.current_user();
    let bot_avatar_url = match bot_user.avatar_url() {
        Some(avatar_url) => avatar_url,
        None => String::new(),
    };
    let chug_default_timeout = env::var("CHUG_TIMEOUT_SECONDS").unwrap().parse::<u64>().unwrap();

    let _ = msg.channel_id.send_message(
        ctx, 
        |message| 
            message
                .reference_message(msg)
                .embed(|embed| embed
                    .author(|author| author.name(&bot_user.name).icon_url(&bot_avatar_url))
                    .title("Command: Chug")
                    .description(
                        format!("The chug command is used to initiate a chug check and see who is ready to play a game.
                        You can initiate a chug check for a single game, or suggest multiple games to see what everyone prefers.
                        You can cancel a chug check early, or alert all chugsters by reacting to the chug check.
                        A chug check lasts {} minutes by default, but can be configured to last shorter or longer by passing in an argument.", chug_default_timeout/60))
                    .color(Color::TEAL)
                    .field(
                        "Usage", 
                        "/chug `{game}`
                        /chug `{duration}` `{game}`
                        /chug `{game 1}`, `{game 2}`, `etc.` 
                        /chug `{duration}`, `{game 1}`, `{game 2}`, `etc.`", 
                        false)
                    .field(
                        "Arguments", 
                        format!("`game x`: The name of a game to suggest for the chug check.
                        `duration`: The duration of the chug check. Defaults to {} minutes if not specified.", chug_default_timeout/60), 
                        false)))
        .await?;
    Ok(())
}

#[command]
async fn image(
    ctx: &Context, 
    msg: &Message, 
    _args: Args, 
) -> CommandResult {
    let bot_user = ctx.cache.current_user();
    let bot_avatar_url = match bot_user.avatar_url() {
        Some(avatar_url) => avatar_url,
        None => String::new(),
    };

    let _ = msg.channel_id.send_message(
        ctx, 
        |message| 
            message
                .reference_message(msg)
                .embed(|embed| embed
                    .author(|author| author.name(&bot_user.name).icon_url(&bot_avatar_url))
                    .title("Command: Image")
                    .description(
                        "The image command is used to generate an image from the provided prompt using an AI image generator.")
                    .color(Color::TEAL)
                    .field(
                        "Usage", 
                        "/image `{prompt}`", 
                        false)
                    .field(
                        "Arguments", 
                        "`prompt`: The prompt to send to the AI image generator.", 
                        false)))
        .await?;
    Ok(())
}

//The following commands need to be suffixed with _help and then have the command name specified in the attribute 
// in order to not conflict with the main command.

#[command("voice")] 
async fn voice_help( 
    ctx: &Context, 
    msg: &Message, 
    _args: Args, 
) -> CommandResult {
    let bot_user = ctx.cache.current_user();
    let bot_avatar_url = match bot_user.avatar_url() {
        Some(avatar_url) => avatar_url,
        None => String::new(),
    };

    let _ = msg.channel_id.send_message(
        ctx, 
        |message| 
            message
                .reference_message(msg)
                .embed(|embed| embed
                    .author(|author| author.name(&bot_user.name).icon_url(&bot_avatar_url))
                    .title("Command: Voice")
                    .description(
                        "The voice command is used to generate speech from the provided prompt using an AI text to speech generator.
                        If the user who runs the command is in a voice channel, the bot will join the voice channel and play the generated speech.
                        If the user who runs the command is not in a voice channel, the bot will reply to the message with the generated speech as an attachment.
                        You will need to specify a voice for the speech generation.
                        A list of voices can be retrieved by running the `voices` command.")
                    .color(Color::TEAL)
                    .field(
                        "Usage", 
                        "/voice `{voice name}`, `{prompt}`", 
                        false)
                    .field(
                        "Arguments", 
                        "`voice name`: The name of the voice to use for the speech generation.
                        `prompt`: The prompt to send to the AI text to speech generator.", 
                        false)))
        .await?;
    Ok(())
}

#[command("voices")]
async fn voices_help(
    ctx: &Context, 
    msg: &Message, 
    _args: Args, 
) -> CommandResult {
    let bot_user = ctx.cache.current_user();
    let bot_avatar_url = match bot_user.avatar_url() {
        Some(avatar_url) => avatar_url,
        None => String::new(),
    };

    let _ = msg.channel_id.send_message(
        ctx, 
        |message| 
            message
                .reference_message(msg)
                .embed(|embed| embed
                    .author(|author| author.name(&bot_user.name).icon_url(&bot_avatar_url))
                    .title("Command: Voices")
                    .description(
                        "The voices command is used to list all the voice names that can be used with the `voice` command.")
                    .color(Color::TEAL)
                    .field(
                        "Usage", 
                        "/voices", 
                        false)
                    .field(
                        "Arguments", 
                        "None", 
                        false)))
        .await?;
    Ok(())
}

#[command("join")]
async fn join_help(
    ctx: &Context, 
    msg: &Message, 
    _args: Args, 
) -> CommandResult {
    let bot_user = ctx.cache.current_user();
    let bot_avatar_url = match bot_user.avatar_url() {
        Some(avatar_url) => avatar_url,
        None => String::new(),
    };

    let _ = msg.channel_id.send_message(
        ctx, 
        |message| 
            message
                .reference_message(msg)
                .embed(|embed| embed
                    .author(|author| author.name(&bot_user.name).icon_url(&bot_avatar_url))
                    .title("Command: Join")
                    .description(
                        "The join command is used to summon the bot to the user's current voice channel.
                        The bot will return an error if the user is not in a voice channel.")
                    .color(Color::TEAL)
                    .field(
                        "Usage", 
                        "/join", 
                        false)
                    .field(
                        "Arguments", 
                        "None", 
                        false)))
        .await?;
    Ok(())
}

#[command("leave")]
async fn leave_help(
    ctx: &Context, 
    msg: &Message, 
    _args: Args, 
) -> CommandResult {
    let bot_user = ctx.cache.current_user();
    let bot_avatar_url = match bot_user.avatar_url() {
        Some(avatar_url) => avatar_url,
        None => String::new(),
    };

    let _ = msg.channel_id.send_message(
        ctx, 
        |message| 
            message
                .reference_message(msg)
                .embed(|embed| embed
                    .author(|author| author.name(&bot_user.name).icon_url(&bot_avatar_url))
                    .title("Command: Leave")
                    .description(
                        "The leave command is used to remove the bot from the user's current voice channel.
                        The bot will return an error if the bot is not currently in a voice channel.
                        The bot will automatically leave a voice channel if all users also leave.")
                    .color(Color::TEAL)
                    .field(
                        "Usage", 
                        "/leave", 
                        false)
                    .field(
                        "Arguments", 
                        "None", 
                        false)))
        .await?;
    Ok(())
}