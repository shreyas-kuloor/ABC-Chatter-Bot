use itertools::Itertools;
use serenity::{
    client::Context,
    model::channel::Message,
    framework::standard::{CommandResult, macros::command, Args},
};

use crate::{models::network_clients::VoiceGenNetworkClient, services::speech_generation_service::get_ai_voices};

#[command]
async fn voices(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let mut data = ctx.data.write().await;
    let client = data.get_mut::<VoiceGenNetworkClient>().unwrap();

    let voices = get_ai_voices(client).await.unwrap();
    let voice_names = voices.iter().map(|v| v.name.clone()).join(", ");
    
    let _ = msg.reply(
        ctx, 
        voice_names)
    .await?;

    Ok(())
}