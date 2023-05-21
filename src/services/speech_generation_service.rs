use std::{error::Error, env};
use bytes::Bytes;
use log::warn;
use reqwest::Method;
use crate::network::eleven_labs::{eleven_labs_network_driver::ElevenLabsClient, eleven_labs_models::{TextToSpeechRequest, VoiceResponse, Voice}};

pub async fn generate_speech_from_prompt(client: &ElevenLabsClient, prompt: String, voice_id: String) -> Result<Option<Bytes>, Box<dyn Error>> {
    let tts_request = TextToSpeechRequest::new(
        prompt, 
        env::var("ELEVEN_LABS_STABILITY").unwrap().parse::<f64>().unwrap(), 
        env::var("ELEVEN_LABS_SIMILARITY_BOOST").unwrap().parse::<f64>().unwrap());

    let tts_response = match client
        .send_request_bytes::<TextToSpeechRequest>(
            format!("text-to-speech/{}", voice_id), 
            Method::POST, 
            Some(tts_request))
        .await{
            Ok(resp) => Some(resp),
            Err(err) => {
                warn!("An unmapped error occurred when generating a voice. {:?}", err);
                None
            },
        };

    Ok(tts_response)
}

pub async fn get_ai_voices(client: &ElevenLabsClient) -> Result<Vec<Voice>, Box<dyn Error>> {
    let voice_response = match client
        .send_request::<VoiceResponse>(
            "voices".to_string(), 
            Method::GET)
        .await{
            Ok(resp) => resp.voices,
            Err(err) => {
                warn!("An unmapped error occurred when retrieving voices. {:?}", err);
                Vec::new()
            },
        };

    Ok(voice_response)
}
