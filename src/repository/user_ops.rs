use mongodb::bson::doc;
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
        let uuid: u32 = rng.gen_range(100_000..=999_999);

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

    pub async fn user_exist(&self, email: &String) -> bool {
        let mut exists = true;

        let filter = doc! {"email": email};
        let user = self
            .users
            .find_one(filter, None)
            .await
            .ok()
            .expect("Failed to get user details");

        if user.is_some() {
            exists = false;
        }
        exists
    }

    pub async fn find_user(&self, id: u32) -> Result<user_model::User, Error> {
        let filter = doc! {"unqid": id};
        let user = self
            .users
            .find_one(filter, None)
            .await
            .ok()
            .expect("Failed to fetch user");

        Ok(user.unwrap())
    }

    //     fix this

    //     pub async fn get_all_users(&self) -> Vec<user_model::User> {
    //         let users = self
    //             .users
    //             .find(None, None)
    //             .await
    //             .ok()
    //             .expect("Failed to retrive users");

    //         // This code could seriously use some error handling... Must be improved
    //         let serial = users.collect().await;

    //         serial
    //     }
}
