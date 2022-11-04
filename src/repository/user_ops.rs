use mongodb::bson::extjson::de::Error;
use mongodb::bson::oid::ObjectId;
use mongodb::results::InsertOneResult;

use rand::Rng;

use crate::api::user;
use crate::model::user_model;
use crate::repository::mongo_repo::MongoRepo;

impl MongoRepo {
    pub async fn register_user(&self, new: user::User) -> Result<InsertOneResult, Error> {
        // generate 6 digit uuid
        let mut rng = rand::thread_rng();
        let uuid: u64 = rng.gen_range(100_000..=999_999);

        let rooms: Vec<ObjectId> = Vec::new();
        let new_doc = user_model::User {
            id: None,
            name: new.name,
            unqid: uuid,
            image: new.image,
            email: new.email,
            rooms: rooms,
        };

        let user = self
            .users
            .insert_one(new_doc, None)
            .await
            .expect("Failed to register user!");

        Ok(user)
    }
}
