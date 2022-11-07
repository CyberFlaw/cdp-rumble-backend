use bson::oid::ObjectId;
use futures::TryStreamExt;
use mongodb::results::CollectionSpecification;
use mongodb::{bson::doc, Collection};

use crate::model::message_model::Messages;
use crate::repository::mongo_repo::MongoRepo;

impl MongoRepo {
    // code for message specific database operations

    // do proper error handling!!!
    pub async fn get_room_coll(&self, room_id: String) -> Collection<Messages> {
        let msg_db = self.message_db.clone();

        let db_cursor = msg_db
            .list_collections(None, None)
            .await
            .ok()
            .expect("Room not found");

        let room_specs: Vec<CollectionSpecification> = db_cursor
            .try_collect()
            .await
            .ok()
            .expect("Failed to fetch collections");

        let mut room_name = String::new();

        for r in room_specs {
            if r.name == room_id {
                room_name = r.name;
            }
        }

        msg_db.collection(room_name.as_str())
    }

    pub async fn add_message(&self, room_id: String, msg: String, owner: u32) {
        let room = self.get_room_coll(room_id).await;

        let new_msg = Messages {
            id: None,
            owner: owner,
            text: msg,
        };

        room.insert_one(new_msg, None)
            .await
            .expect("Failed to store msg");
    }

    pub async fn get_all_messages(&self, room_id: String) -> Vec<Messages> {
        let msgs_coll = self.get_room_coll(room_id).await;
        let msgs = msgs_coll
            .find(None, None)
            .await
            .ok()
            .expect("Failed to retrive messages");

        let serial: Vec<Messages> = msgs.try_collect().await.unwrap();

        serial
    }

    pub async fn find_message(
        &self,
        room_id: String,
        msg_id: ObjectId,
    ) -> (Collection<Messages>, Messages) {
        let msg_coll = self.get_room_coll(room_id).await;

        let filter = doc! {"_id": msg_id};
        let msg = msg_coll
            .find_one(filter, None)
            .await
            .ok()
            .expect("Message don't exist");

        (msg_coll, msg.unwrap())
    }

    pub async fn update_message(&self, room_id: String, msg_id: ObjectId, text: String) {
        let msg_data = self.find_message(room_id, msg_id).await;

        let filter = doc! {
            "_id": msg_data.1.id
        };

        let update_doc = doc! {
            "$set": {
                "_id": msg_data.1.id,
                "owner": msg_data.1.owner,
                "text": text
            }
        };

        msg_data
            .0
            .update_one(filter, update_doc, None)
            .await
            .ok()
            .expect("Failed to edit message");
    }

    pub async fn delete_message(&self, room_id: String, msg_id: ObjectId) {
        let msg_data = self.find_message(room_id, msg_id).await;

        let filter = doc! {
            "_id": msg_data.1.id
        };

        msg_data
            .0
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Failed to delete user");
    }
}
