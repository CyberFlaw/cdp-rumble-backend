extern crate dotenv;

use crate::model::{room_model::Rooms, user_model::User};
use dotenv::dotenv;
use mongodb::{
    // results::InsertOneResult,
    Client,
    Collection,
};
use std::env;

pub struct MongoRepo {
    pub users: Collection<User>,
    pub rooms: Collection<Rooms>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error Loading .env"),
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("Rumble");
        let user_col: Collection<User> = db.collection("Users");
        let room_col: Collection<Rooms> = db.collection("Rooms");

        MongoRepo {
            users: user_col,
            rooms: room_col,
        }
    }
}
