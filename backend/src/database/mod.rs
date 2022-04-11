use mongodb::{bson, Client, Cursor, Database, options::FindOptions};
use mongodb::bson::{doc, Document, DateTime};
use mongodb::options::{ClientOptions};
use rocket::fairing::{AdHoc};
use mongodb::results::{InsertOneResult};
use rocket::futures::TryStreamExt;
use crate::model::{GameResult, JsonGameResult, Leaderboard, ComputerStatistics};

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
        let mut highest_id: i64 = 0;
        while let Some(doc) = cursor.try_next().await? {
            // println!("doc: {:#?}", doc);
            highest_id = doc._id.unwrap();
            break;
        }
        // let highest_id: i64 = highest_id.parse::<i64>().unwrap();
        let next_id = highest_id + 1;
        println!("Next ID: {}", next_id);

        game_result._id = Some(next_id);
        let insert: InsertOneResult = collection.insert_one(game_result, None).await?;
        Ok(insert.inserted_id.to_string())
    }

    pub async fn fetch_all_game_results(&self, winner_name: Option<String>) -> mongodb::error::Result<Vec<JsonGameResult>> {
        let collection = self.database.collection::<GameResult>(self.game_results_col);
        let filter = match winner_name {
            Some(name) => Some(doc! { "winner_name": name }),
            None => None
        };
        let mut cursor: Cursor<GameResult> = collection.find(filter, None).await?;
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

    pub async fn get_leaderboard(&self, difficulty: Option<String>) -> mongodb::error::Result<Vec<Leaderboard>> {
        let collection = self.database.collection::<GameResult>(self.game_results_col);
        // we need to group by winner name and retrieve the winner name and size of each group
        let mut stage_filter_difficulty = doc! {
            "$match": { 
                "is_draw": false
            }
        };
        if let Some(diff) = difficulty {
            stage_filter_difficulty = doc! {
                "$match": {
                    "is_draw": false,
                    "difficulty": diff
                }
            }
        }
        let stage_group_winner = doc! {
            "$group": {
                "_id": "$winner_name",
                // count the number of wins in the group
                "wins": { "$sum": 1 },
            }
        };
        let stage_sort_desc = doc! {
            "$sort": {"wins": -1}
        };
        let stage_rename_field = doc! {
            "$project": {
                "winner_name": "$_id",
                "wins": "$wins"
            }
        };
        let pipeline = vec![stage_filter_difficulty, stage_group_winner, stage_sort_desc, stage_rename_field];
        let mut results = collection.aggregate(pipeline, None).await?;
        let mut leaders: Vec<Leaderboard> = Vec::new();
        while let Some(result) = results.try_next().await? {
            let doc: Leaderboard = bson::from_document(result)?;
            println!("* {:?}", doc);
            leaders.push(doc);
        }
        Ok(leaders)
    }

    pub async fn get_comp_stats(&self) -> mongodb::error::Result<Vec<ComputerStatistics>> {
        let collection = self.database.collection::<GameResult>(self.game_results_col);
        let hard_played = collection.count_documents(
            doc! {
                "p2_name": "Computer",
                "difficulty": "Hard"
            },
            None
        ).await?;
        let hard_won = collection.count_documents(
            doc! {
                "p2_name": "Computer",
                "difficulty": "Hard",
                "winner_name": "Computer"
            },
            None
        ).await?;
        let medium_played = collection.count_documents(
            doc! {
                "p2_name": "Computer",
                "difficulty": "Medium"
            },
            None
        ).await?;
        let medium_won = collection.count_documents(
            doc! {
                "p2_name": "Computer",
                "difficulty": "Medium",
                "winner_name": "Computer"
            },
            None
        ).await?;
        let easy_played = collection.count_documents(
            doc! {
                "p2_name": "Computer",
                "difficulty": "Easy"
            },
            None
        ).await?;
        let easy_won = collection.count_documents(
            doc! {
                "p2_name": "Computer",
                "difficulty": "Easy",
                "winner_name": "Computer"
            },
            None
        ).await?;
        Ok(vec![ComputerStatistics {
            hard_played,
            hard_won,
            medium_played,
            medium_won,
            easy_played,
            easy_won,
        }])
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