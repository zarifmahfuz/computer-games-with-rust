#[macro_use] extern crate rocket;
mod database;
mod model;
mod utils;
use rocket::serde::{json::Json};
use rocket::response::{status::Created};
use std::error::Error;
use rocket::State;
use crate::model::{GameResult, JsonGameResult, Leaderboard, ComputerStatistics};
use crate::utils::GenericError;

#[post("/gameresults", format = "json", data = "<game_result>")]
async fn create(game_result: Json<JsonGameResult>, database: &State<database::MongoDB>) -> Result<Created<Option<String>>, ()> {
    println!("POST /gameresults received: {:?}", game_result.0);
    let created_id = database.add_game_result(&mut GameResult::from(game_result.0)).await.ok();
    Ok(Created::new("/gameresults").body(created_id))
}

#[get("/gameresults")]
async fn list(database: &State<database::MongoDB>) -> Result<Json<Vec<JsonGameResult>>, GenericError> {
    match database.fetch_all_game_results(None).await {
        Ok(game_results) => Ok(Json(game_results)),
        Err(error) => Err(GenericError::new(&*format!("{:?}", error)))
    }
}

#[get("/gameresults?<winner_name>")]
async fn list_by_winner(winner_name: String, database: &State<database::MongoDB>) -> Result<Json<Vec<JsonGameResult>>, GenericError> {
    println!("Route works, name: {}", winner_name);
    match database.fetch_all_game_results(Some(winner_name)).await {
        Ok(game_results) => Ok(Json(game_results)),
        Err(error) => Err(GenericError::new(&*format!("{:?}", error)))
    }
}

#[delete("/gameresults")]
async fn delete(database: &State<database::MongoDB>) -> Result<(), GenericError> {
    match database.delete_all_game_results().await {
        Ok(_) => Ok(()),
        Err(error) => Err(GenericError::new(&*format!("{:?}", error)))
    }
}

#[get("/leaderboard")]
async fn list_top_players(database: &State<database::MongoDB>) -> Result<Json<Vec<Leaderboard>>, GenericError>  {
    match database.get_leaderboard(None).await {
        Ok(leaders) => Ok(Json(leaders)),
        Err(error) => Err(GenericError::new(&*format!("{:?}", error)))
    }
}

#[get("/leaderboard?<difficulty>")]
async fn list_top_players_by_difficulty(difficulty: String, database: &State<database::MongoDB>) 
    -> Result<Json<Vec<Leaderboard>>, GenericError>  {
    match database.get_leaderboard(Some(difficulty)).await {
        Ok(leaders) => Ok(Json(leaders)),
        Err(error) => Err(GenericError::new(&*format!("{:?}", error)))
    }
}

#[get("/computer_stats")]
async fn computer_stats(database: &State<database::MongoDB>) 
    -> Result<Json<Vec<ComputerStatistics>>, GenericError> {
    match database.get_comp_stats().await {
        Ok(stats) => Ok(Json(stats)),
        Err(error) => Err(GenericError::new(&*format!("{:?}", error)))
    }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // before Rocket can dispatch requests to a route, the route needs to be mounted
    rocket::build()
        .attach(database::init().await) // connect to the database
        .mount("/api", routes![create, list, delete, list_by_winner, list_top_players, list_top_players_by_difficulty, computer_stats])
        .launch()
        .await?;
    Ok(())
}