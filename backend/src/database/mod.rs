use mongodb::{bson, Client, Cursor, Database, options::FindOptions};
use mongodb::bson::{doc, Document, DateTime};
use mongodb::options::{ClientOptions};
use rocket::fairing::{AdHoc};
use mongodb::results::{InsertOneResult};
use rocket::futures::TryStreamExt;
use crate::model::{GameResult, JsonGameResult};

#[derive(Debug)]
pub struct MongoDB {
    database: Database,
    game_results_col: &'static str,
}

impl MongoDB {
    fn new(database: Database) -> Self {
        MongoDB {
            database,
            game_results_col: "GameResults",
        }
    }

    pub async fn add_game_result(&self, game_result: &mut GameResult) -> mongodb::error::Result<String> {
        let collection = self.database.collection::<GameResult>(self.game_results_col);
        // get the highest game number currently in the collection
        let find_options = FindOptions::builder()
            .sort(doc! { "_id": -1 })
            .limit(1)
            .build();
        let mut cursor = collection.find(None, find_options).await?;
        let mut highest_id: String = String::from("0");
        while let Some(doc) = cursor.try_next().await? {
            highest_id = doc._id.unwrap();
            break;
        }
        let highest_id: i64 = highest_id.parse::<i64>().unwrap();
        let next_id = highest_id + 1;
        println!("Next ID: {}", next_id);

        game_result._id = Some(next_id.to_string());
        let insert: InsertOneResult = collection.insert_one(game_result, None).await?;
        Ok(insert.inserted_id.to_string())
    }

    pub async fn fetch_all_game_results(&self) -> mongodb::error::Result<Vec<JsonGameResult>> {
        let collection = self.database.collection::<GameResult>(self.game_results_col);
        let mut cursor: Cursor<GameResult> = collection.find(doc! {}, None).await?;
        let mut results: Vec<JsonGameResult> = Vec::new();
        while let Some(result) = cursor.try_next().await? {
            results.push(JsonGameResult::from(result));
        }
        Ok(results)
    }

    pub async fn delete_all_game_results(&self) -> mongodb::error::Result<()> {
        let collection = self.database.collection::<GameResult>(self.game_results_col);
        collection.delete_many(doc! {}, None).await?;
        Ok(())
    }
}

pub async fn init() -> AdHoc {
    AdHoc::on_ignite("Connect to MongoDB cluster", |rocket| async {
        match connect().await {
            Ok(database) => {
                rocket.manage(MongoDB::new(database))
            }
            Err(error) => {
                panic!("Cannot connect to MDB instance:: {:?}", error)
            }
        }
    })
}

async fn connect() -> mongodb::error::Result<Database> {
    // let mdb_uri = std::env::var("MDB_URL").or(Err("MDB_URL environment variable missing")).unwrap();
    let mdb_uri = String::from("mongodb://localhost:27017");
    let client_options = ClientOptions::parse(mdb_uri).await?;
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    Ok(client.database("ece421"))
}