use std::{error::Error, env};
use log::{info, warn};
use reqwest::Method;
use crate::{
    network::games::{igdb_network_driver::IGDBClient, igdb_models::GameResponse},
    errors::network_error::NetworkErrorType,
};

pub async fn get_game_cover_url_by_name(client: &mut IGDBClient, game_name: String) -> Result<Option<String>, Box<dyn Error>> {
    let game_request = format!("fields cover.*; search \"{}\";", game_name);
    let cover_image_url = match client.send_request::<String, Vec<GameResponse>>("games".to_string(), Method::POST, Some(game_request)).await {
        Ok(game_response) => {
            if let Some(first_game) = game_response.first() {
                if let Some(first_game_cover) = &first_game.cover {
                    Some(format!(
                        "{}/t_{}/{}.png", 
                        env::var("GAME_IMAGE_BASE_URL").unwrap(), 
                        env::var("GAME_IMAGE_SIZE").unwrap(), 
                        first_game_cover.image_id))
                }
                else {
                    None
                }
            }
            else {
                None
            }
            
        },
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
