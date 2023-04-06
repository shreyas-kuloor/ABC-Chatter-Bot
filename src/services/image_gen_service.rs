use std::{error::Error, env};
use log::warn;
use reqwest::Method;

use crate::network::stable_diffusion::{stable_diffusion_models::{StableDiffusionTextRequest, StableDiffusionResponse}, stable_diffusion_network_driver::StableDiffusionClient};

pub async fn generate_image_base64_from_prompt(client: &StableDiffusionClient, prompt: String) -> Result<Option<String>, Box<dyn Error>> {
    let image_gen_request = StableDiffusionTextRequest::new(prompt, env::var("STABLE_DIFFUSION_SAMPLING_STEPS").unwrap().parse::<u64>().unwrap());

    let image_string_base64 = match client
        .send_request::<StableDiffusionTextRequest, StableDiffusionResponse>(
            "txt2img".to_string(), 
            Method::POST, 
            Some(image_gen_request))
        .await{
            Ok(resp) => Some(resp.images[0].clone()),
            Err(err) => {
                warn!("An unmapped error occurred when generating an image. {:?}", err);
                None
            },
        };

    Ok(image_string_base64)
}