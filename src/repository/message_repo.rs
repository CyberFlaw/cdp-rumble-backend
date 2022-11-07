use mongodb::{Client, Collection, Database};
use std::error::Error;
use std::os::unix::prelude::OwnedFd;

use crate::model::message_model::Messages;
use crate::repository::mongo_repo::MongoRepo;

impl MongoRepo {
    // code for message specific database operations
    pub async fn add_chat_room(&mut self, room_id: String) {
        let message_db = self.message_db.clone();
        let new_room: Collection<Messages> = message_db
            .collection(format!("{}", room_id).as_str())
            .clone();

        self.activated_rooms.insert(room_id, new_room);
    }

    // do proper error handling!!!
    pub async fn get_room_coll(&self, room_id: String) -> Collection<Messages> {
        let room = self.activated_rooms.get(&room_id).unwrap().clone();
        room
    }

    // This is implemented in a way such that every single message is synced with db real-time
    // This should be replaced with a buffer and this function should only work ever 10 msg or one of the user disconects
    pub async fn add_message(&self, room_id: String, msg: String, owner: u32) {
        let room = self.activated_rooms.get(&room_id).unwrap().clone();
        let new_msg = Messages {
            id: None,
            owner: owner,
            text: msg,
        };

        room.insert_one(new_msg, None)
            .await
            .expect("Failed to store msg");
    }
}
