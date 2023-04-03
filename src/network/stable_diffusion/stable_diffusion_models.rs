use serde::{
    Serialize, 
    Deserialize
};

#[derive(Serialize, Deserialize, Debug)]
pub struct StableDiffusionTextRequest {
    pub prompt: String,
}

impl StableDiffusionTextRequest {
    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StableDiffusionResponse {
    pub images: Vec<String>,
}