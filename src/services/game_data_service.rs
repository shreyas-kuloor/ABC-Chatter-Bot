use std::{error::Error, env};
use log::{info, warn};
use crate::{
    network::games::igdb_network_driver::IGDBClient,
    errors::network_error::NetworkErrorType,
};

pub async fn get_game_cover_url_by_name(client: &mut IGDBClient, game_name: String) -> Result<Option<String>, Box<dyn Error>> {
    let cover_image_url = match client.post_game_cover_details(game_name).await {
        Ok(game_response) => Some(format!(
            "{}/{}/{}.png", 
            env::var("GAME_IMAGE_BASE_URL").unwrap(), 
            env::var("GAME_IMAGE_SIZE").unwrap(), 
            game_response.cover.image_id)),
        Err(err) => match err.error_type {
            NetworkErrorType::Unauthorized => {
                client.refresh_bearer_token().await.unwrap();
                info!("IGDB Access Token expired. Refreshing.");
                None
            },
            NetworkErrorType::Unknown => {
                warn!("An unknown error occurred when fetching game cover details.");
                None
            },
            _ => {
                warn!("An unmapped error occurred when fetching game cover details.");
                None
            },
        }
    };

    Ok(cover_image_url)
}
