use rocket::serde::{Serialize, Deserialize, self};
use mongodb::bson::{DateTime};
use mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GameResult {
    pub _id: Option<i64>,
    pub game_type: String,
    pub p1_name: String,
    pub p2_name: String,
    pub is_draw: bool,
    pub winner_name: String,
    pub difficulty: Option<String>,
    pub date_time: DateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct JsonGameResult {
    pub _id: Option<i64>,
    pub game_type: String,
    pub p1_name: String,
    pub p2_name: String,
    pub is_draw: bool,
    pub winner_name: String,
    pub difficulty: Option<String>,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub date_time: DateTime,
}

impl From<JsonGameResult> for GameResult {
    fn from(other: JsonGameResult) -> GameResult {
        GameResult {
            _id: other._id,
            game_type: other.game_type,
            p1_name: other.p1_name,
            p2_name: other.p2_name,
            is_draw: other.is_draw,
            winner_name: other.winner_name,
            difficulty: other.difficulty,
            date_time: other.date_time,
        }
    }
}
  
impl From<GameResult> for JsonGameResult {
    fn from(other: GameResult) -> JsonGameResult {
        JsonGameResult {
            _id: other._id,
            game_type: other.game_type,
            p1_name: other.p1_name,
            p2_name: other.p2_name,
            is_draw: other.is_draw,
            winner_name: other.winner_name,
            difficulty: other.difficulty,
            date_time: other.date_time,
        }
    }
}

// holds grouped data for a leaderboard request
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Leaderboard {
    #[serde(rename = "winner_name")]
    _id: String,
    #[serde(default)]
    wins: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ComputerStatistics {
    #[serde(default)]
    pub hard_played: u64,
    #[serde(default)]
    pub hard_won: u64,
    #[serde(default)]
    pub medium_played: u64,
    #[serde(default)]
    pub medium_won: u64,
    #[serde(default)]
    pub easy_played: u64,
    #[serde(default)]
    pub easy_won: u64,
}