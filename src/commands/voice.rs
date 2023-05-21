use std::{fs, borrow::Cow};
use log::{warn, debug};
use serenity::{
    client::Context,
    model::{channel::Message, prelude::AttachmentType},
    framework::standard::{CommandResult, macros::command, Args},
};
use songbird::{Event, TrackEvent};

use crate::{models::network_clients::VoiceGenNetworkClient, services::voice_gen_service::{get_ai_voices, gen_voice_from_prompt}, utils::voice_utils::TrackEndNotifier};

#[command]
async fn voice(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut data = ctx.data.write().await;
    let client = data.get_mut::<VoiceGenNetworkClient>().unwrap();

    let voice_name = args.single::<String>().unwrap();
    let prompt = args.rest().to_string();

    let voices = get_ai_voices(client).await.unwrap();
    if let Some(matching_voice) = voices.iter().find(|v| v.name == voice_name) {
        let audio_bytes_response = gen_voice_from_prompt(client, prompt, matching_voice.voice_id.clone()).await.unwrap();
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
            let manager = songbird::get(ctx).await.ok_or_else(|| warn!("Could not get songbird.")).unwrap().clone();
            debug!("Songbird retrieved.");
            let (handle_lock, success) = manager.join(guild_id, channel_to_connect).await;

            if let Ok(_channel) = success {
                debug!("Joined channel: {:?}", channel_to_connect);
                let mut handler = handle_lock.lock().await;

                fs::write("voice.mp3", &audio_bytes).unwrap();
                debug!("Saved voice.");

                handler.add_global_event(
                    Event::Track(TrackEvent::End),
                    TrackEndNotifier {
                        guild_id: guild.id,
                        ctx: ctx.clone(),
                    },
                );

                let source = match songbird::ffmpeg("voice.mp3").await {
                    Ok(source) => source,
                    Err(err) => {
                        warn!("Error starting voice source: {:?}", err);
                        return Ok(());
                    }
                };

                handler.play_source(source);
            }
            else {
                warn!("Error joining channel.");
            }
        }
        else {
            warn!("Voice creation failed.")
        }
    } 
    else {
        let _ = msg.channel_id.send_message(
            ctx, 
            |create_msg| 
                create_msg
                .reference_message(msg)
                .content("A voice with that name was not found. Please try again with a valid voice."))
        .await?;
    }
    Ok(())
}