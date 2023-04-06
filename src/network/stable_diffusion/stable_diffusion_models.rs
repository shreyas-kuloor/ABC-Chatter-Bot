use serde::{
    Serialize, 
    Deserialize
};

#[derive(Serialize, Deserialize, Debug)]
pub struct StableDiffusionTextRequest {
    pub prompt: String,
    pub steps: u64,
}

impl StableDiffusionTextRequest {
    pub fn new(prompt: String, steps: u64) -> Self {
        Self {
            prompt,
            steps,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StableDiffusionResponse {
    pub images: Vec<String>,
}