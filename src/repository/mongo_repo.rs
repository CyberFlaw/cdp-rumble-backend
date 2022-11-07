extern crate dotenv;
use crate::model::{rooms_model::Rooms, user_model::User};

use dotenv::dotenv;
use mongodb::{Client, Collection, Database};
use std::env;

pub struct MongoRepo {
    pub reg_col: Collection<User>,
    pub rooms_col: Collection<Rooms>,

    pub users_db: Database,
    pub message_db: Database,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error Loading .env"),
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let user_db = client.database("Users");
        let user_col: Collection<User> = user_db.collection("Registered");
        let room_col: Collection<Rooms> = user_db.collection("Rooms");

        let message_db = client.database("Messages");

        MongoRepo {
            reg_col: user_col,
            rooms_col: room_col,

            users_db: user_db,
            message_db: message_db,
        }
    }
}
