#[macro_use] extern crate rocket;
mod database;
mod model;
mod utils;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::response::{Debug, status::Created};
use std::error::Error;
use rocket::State;
use crate::model::{GameResult};
use crate::utils::GenericError;

#[post("/gameresults", format = "json", data = "<game_result>")]
async fn create(mut game_result: Json<GameResult>, database: &State<database::MongoDB>) -> Result<Created<Json<GameResult>>, ()> {
    database.add_game_result(&mut game_result.0).await.ok();
    println!("POST /gameresults received: {:?}", game_result.0);
    Ok(Created::new("/gameresults").body(game_result))
}

#[get("/gameresults")]
async fn list(database: &State<database::MongoDB>) -> Result<Json<Vec<GameResult>>, GenericError> {
    // let mut game_results: Vec<GameResult> = database.fetch_all_game_results().await?;
    // Ok(Json(game_results))
    match database.fetch_all_game_results().await {
        Ok(game_results) => Ok(Json(game_results)),
        Err(error) => Err(GenericError::new(&*format!("{:?}", error)))
    }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // before Rocket can dispatch requests to a route, the route needs to be mounted
    rocket::build()
        .attach(database::init().await) // connect to the database
        .mount("/", routes![create, list])
        .launch()
        .await?;
    Ok(())
}