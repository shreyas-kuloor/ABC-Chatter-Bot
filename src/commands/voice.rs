use std::{fs, borrow::Cow};
use log::warn;
use serenity::{
    client::Context,
    model::{channel::Message, prelude::AttachmentType},
    framework::standard::{CommandResult, macros::command, Args},
};

use crate::{models::network_clients::VoiceGenNetworkClient, services::speech_generation_service::{get_ai_voices, generate_speech_from_prompt}};

#[command]
async fn voice(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut data = ctx.data.write().await;
    let client = data.get_mut::<VoiceGenNetworkClient>().unwrap();

    let voice_name = args.single::<String>().unwrap();
    let prompt = args.rest().to_string();

    let voices = get_ai_voices(client).await.unwrap();
    if let Some(matching_voice) = voices.iter().find(|v| v.name == voice_name) {
        let audio_bytes_response = generate_speech_from_prompt(client, prompt, matching_voice.voice_id.clone()).await.unwrap();
        if let Some(audio_bytes) =  audio_bytes_response{
            let guild = msg.guild(&ctx.cache).unwrap();
            let guild_id = guild.id;
            let channel_id = guild
                .voice_states.get(&msg.author.id)
                .and_then(|voice_state| voice_state.channel_id);

            let channel_to_connect = match channel_id {
                Some(channel) => channel,
                None => {
                    let attachment_type = AttachmentType::Bytes { data: Cow::Borrowed(&audio_bytes), filename: "voice.mp3".to_string() };
                    let _ = msg.channel_id.send_message(ctx, |create_msg| create_msg.reference_message(msg).add_file(attachment_type)).await?;

                    return Ok(());
                }
            };

            drop(data); // songbird::get() internally uses the TypeMap, so we need to drop our existing lock to prevent deadlock.
            let manager = songbird::get(ctx).await.unwrap().clone();
            let (handle_lock, success) = manager.join(guild_id, channel_to_connect).await;

            if let Ok(_channel) = success {
                let mut handler = handle_lock.lock().await;

                fs::write("voice.mp3", &audio_bytes).unwrap();

                let source = match songbird::ffmpeg("voice.mp3").await {
                    Ok(source) => source,
                    Err(err) => {
                        warn!("Error starting voice source: {:?}", err);
                        return Ok(());
                    }
                };

                handler.enqueue_source(source);
            }
            else {
                warn!("Error joining channel.");
                let _ = msg.reply(
                    ctx, 
                    "Sorry, I was unable to join the voice channel.")
                .await?;
            }
        }
        else {
            warn!("Voice creation failed.");
            let _ = msg.reply(
                ctx, 
                "Sorry, something went wrong during voice creation.")
            .await?;
        }
    } 
    else {
        let _ = msg.reply(
            ctx, 
            "A voice with that name was not found. Please try again with a valid voice.")
        .await?;
    }
    Ok(())
}