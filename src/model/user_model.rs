use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// use crate::common::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        // common::serialize_with = "serialize_object_id"
    )]
    pub id: Option<ObjectId>,
    pub name: String,
    pub unqid: u32,
    pub image: String,
    pub rooms: Vec<ObjectId>,
    pub email: String,
}
