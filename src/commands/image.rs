use std::borrow::Cow;

use base64::{engine::general_purpose, Engine};
use log::warn;
use serenity::{
    client::Context,
    model::{channel::Message, prelude::AttachmentType},
    framework::standard::{CommandResult, macros::command, Args},
};

use crate::{models::network_clients::ImageGenNetworkClient, services::image_gen_service::generate_image_base64_from_prompt};

#[command]
async fn image(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut data = ctx.data.write().await;
    let client = data.get_mut::<ImageGenNetworkClient>().unwrap();

    let image_gen_prompt = args.message();

    let image_response_b64 = generate_image_base64_from_prompt(client, image_gen_prompt.to_string()).await.unwrap();

    if let Some(image_b64) = image_response_b64 {
        if let Ok(image_bytes) = general_purpose::STANDARD.decode(&image_b64) {
            let attachment_type = AttachmentType::Bytes { data: Cow::Borrowed(&image_bytes), filename: "gen_image.png".to_string() };

            let _ = msg.channel_id.send_message(ctx, |create_msg| create_msg.reference_message(msg).add_file(attachment_type)).await?;
        }
        else {
            warn!("Error occurred while trying to decode image base64. {}", &image_b64);
        }
    }
    else {
        warn!("None image base64 was returned.");
    }
    Ok(())
}