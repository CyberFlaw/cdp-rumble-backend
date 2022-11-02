extern crate dotenv;

use crate::model::user_model::User;
use dotenv::dotenv;
// use futures::TryFutureExt;
use mongodb::{
    // bson::extjson::de::Error,
    // results::InsertOneResult,
    Client,
    Collection,
};
use std::env;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error Loading .env"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rumble");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }
}
