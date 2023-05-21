use serde::{
    Serialize, 
    Deserialize
};

#[derive(Serialize, Deserialize, Debug)]
pub struct TextToSpeechVoiceSettingsRequest {
    stability: f64,
    similarity_boost: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextToSpeechRequest {
    pub text: String,
    pub model_id: String,
    pub voice_settings: TextToSpeechVoiceSettingsRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoiceResponse {
    pub voices: Vec<Voice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Voice {
    pub voice_id: String,
    pub name: String,
}

impl TextToSpeechVoiceSettingsRequest {
    fn new(stability: f64, similarity_boost: f64) -> Self {
        Self {
            stability,
            similarity_boost,
        }
    }
}

impl TextToSpeechRequest {
    pub fn new(prompt: String, stability: f64, similarity_boost: f64) -> Self {
        Self {
            text: prompt,
            model_id: "eleven_monolingual_v1".to_string(),
            voice_settings: TextToSpeechVoiceSettingsRequest::new(stability, similarity_boost),
        }
    }
}
