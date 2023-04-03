use serde::{
    Serialize, 
    Deserialize
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoverResponse {
    pub id: i64,
    pub game: i64,
    pub height: i64,
    pub image_id: String,
    pub url: String,
    pub width: i64,
    pub checksum: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameResponse {
    pub id: i64,
    pub cover: Option<CoverResponse>,
}
