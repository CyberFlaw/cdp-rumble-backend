use mongodb::bson::doc;
use mongodb::bson::extjson::de::Error;
use mongodb::results::InsertOneResult;

use crate::model::rooms_model;
use crate::repository::mongo_repo::MongoRepo;

impl MongoRepo {
    // function to add room to collection
    pub async fn register_room(
        &self,
        user1: u32,
        user2: u32,
        name: String,
    ) -> Result<InsertOneResult, Error> {
        let new_doc = rooms_model::Rooms {
            id: None,
            user1: user1,
            user2: user2,
            name: name,
        };

        let room = self
            .rooms_col
            .insert_one(new_doc, None)
            .await
            .expect("Failed to register room!");

        Ok(room)
    }

    // function to check if room exits
    pub async fn room_exists(&self, name: &String) -> bool {
        let mut exists = true;

        let filter = doc! {"name": name};
        let user = self
            .rooms_col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Failed to get user details");

        if user.is_some() {
            exists = false;
        }
        exists
    }

    //     function to get details for a room with given room name
    pub async fn find_room(&self, name: &String) -> Result<rooms_model::Rooms, Error> {
        let filter = doc! {"name": name};
        let room = self
            .rooms_col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Failed to fetch user");

        Ok(room.unwrap())
    }
}
