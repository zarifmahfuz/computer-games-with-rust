#[macro_use] extern crate rocket;
mod database;
mod model;
mod utils;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::response::{Debug, status::Created};
use std::error::Error;
use rocket::State;
use crate::model::{GameResult, JsonGameResult};
use crate::utils::GenericError;

#[post("/gameresults", format = "json", data = "<game_result>")]
async fn create(game_result: Json<JsonGameResult>, database: &State<database::MongoDB>) -> Result<Created<Option<String>>, ()> {
    println!("POST /gameresults received: {:?}", game_result.0);
    let created_id = database.add_game_result(&mut GameResult::from(game_result.0)).await.ok();
    Ok(Created::new("/gameresults").body(created_id))
}

#[get("/gameresults")]
async fn list(database: &State<database::MongoDB>) -> Result<Json<Vec<JsonGameResult>>, GenericError> {
    match database.fetch_all_game_results().await {
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

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // before Rocket can dispatch requests to a route, the route needs to be mounted
    rocket::build()
        .attach(database::init().await) // connect to the database
        .mount("/", routes![create, list, delete])
        .launch()
        .await?;
    Ok(())
}