use std::error::Error;

use log::{warn, info};
use serenity::{model::voice::VoiceState, prelude::Context};

pub async fn leave_empty_voice_channel(ctx: &Context, old: Option<VoiceState>) -> Result<(), Box<dyn Error>> {    
    if let Some(old_voice_state) = old {
        if let Some(old_guild_id) = old_voice_state.guild_id {
            if let Some(old_guild) = old_guild_id.to_guild_cached(&ctx.cache) {
                let bot_user_id = &ctx.cache.current_user_id();
                let channel_id = old_guild
                    .voice_states.get(bot_user_id)
                    .and_then(|voice_state| voice_state.channel_id);

                if let Some(bot_channel_id) = channel_id {
                    let bot_channel = bot_channel_id.to_channel(&ctx.http).await.unwrap();
                    let bot_channel_user_count = bot_channel.guild().unwrap().members(&ctx.cache).await.unwrap().len();
                    if bot_channel_user_count <= 1 {
                        let manager = songbird::get(ctx).await.unwrap().clone();
                        let has_handler = manager.get(old_guild_id).is_some();

                        if has_handler {
                            if let Err(err) = manager.remove(old_guild_id).await {
                                warn!("Error occurred while trying to leave voice channel: {:?}", err);
                            }
                            
                            info!("Left voice channel because all other users have left.");
                        }
                        else {
                            warn!("Track ended while not in a voice channel.");
                        }
                    }
                }
            }
        }
    }

    Ok(())
}