use rocket::serde::{Serialize, Deserialize, self};
use mongodb::bson::{DateTime};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GameResult {
    // #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    // pub game_number: Option<String>,    // unnecessary
    pub game_type: String,
    pub p1_name: String,
    pub p2_name: String,
    pub is_draw: bool,
    pub winner_name: String,
    // #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub difficulty: Option<String>,
    pub date_time: Option<CustomDateTime>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum CustomDateTime {
    BSonFormat(DateTime),
    StringFormat(String)
}