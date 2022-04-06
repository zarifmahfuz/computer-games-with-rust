use mongodb::{bson, Client, Cursor, Database, options::FindOptions};
use mongodb::bson::{doc, Document, DateTime};
use mongodb::options::{ClientOptions};
use rocket::fairing::{AdHoc};
use mongodb::results::{InsertOneResult};
use rocket::futures::TryStreamExt;
use crate::model::{GameResult, CustomDateTime};

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
        game_result.date_time = Some(CustomDateTime::BSonFormat(bson::DateTime::now()));
        let insert: InsertOneResult = collection.insert_one(game_result, None).await?;
        Ok(insert.inserted_id.to_string())
    }

    pub async fn fetch_all_game_results(&self) -> mongodb::error::Result<Vec<GameResult>> {
        let collection = self.database.collection::<GameResult>(self.game_results_col);
        let find_options = FindOptions::builder()
            .build();

        let mut cursor: Cursor<GameResult> = collection.find(doc! {}, find_options).await?;

        let mut results: Vec<GameResult> = Vec::new();
        while let Some(result) = cursor.try_next().await? {
            results.push(result);
        }
        let _: Vec<&mut GameResult> = results.iter_mut().map(|result| {
            // result.date_time = Some(result.date_time.unwrap().to_rfc3339_string())
            let mut formatted_dt: String = String::from("");
            if let Some(dt) = &result.date_time {
                match dt {
                    CustomDateTime::BSonFormat(bson_dt) => {
                        formatted_dt = bson_dt.to_rfc3339_string();
                    }
                    _ => { }
                }
            }
            result.date_time = Some(CustomDateTime::StringFormat(formatted_dt));
            result
        }).collect();
        Ok(results)
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