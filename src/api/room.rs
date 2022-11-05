use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, Result};
// use log::debug;
use serde::{Deserialize, Serialize};

use crate::repository::mongo_repo::MongoRepo;

#[derive(Serialize, Deserialize)]
struct Message {
    msg: String,
    owner: i64,
}

#[derive(Deserialize)]
pub struct AddQuery {
    user: u32,
    friend: u32,
}

// add room to the db
// Querry parametrs should be used instead of explicit routes /join?usr={Userid}&frnd={Userid}
#[post("/join")]
pub async fn add_room(info: web::Query<AddQuery>, req: HttpRequest) -> Result<impl Responder> {
    let user_str = info.user.to_string();
    let friend_str = info.friend.to_string();

    // db driver code to (insert) create a new record in Rooms collection in Registered db
    let db = req.app_data::<web::Data<MongoRepo>>().unwrap();
    let exists = db.room_exists(&format!("{}{}", user_str, friend_str)).await;

    // return _insert from the if and return it as impl Responder, to give valid HTTP responce
    if exists {
        let _insert = db
            .register_room(
                info.user,
                info.friend,
                format!("{}{}", user_str, friend_str),
            )
            .await
            .unwrap();
    }

    // db driver code to (update) push the room id to both users [room] attribute

    let responce_str = format!(
        "Room created with room name: {}",
        format!("{}{}", info.user, info.friend)
    )
    .to_string();
    Ok(HttpResponse::Ok().body(responce_str))
}

// fectch room details
#[get("/room/{name}")]
pub async fn fetch_room_data(req: HttpRequest, path: web::Path<String>) -> Result<impl Responder> {
    let name = path.into_inner();

    let db = req.app_data::<web::Data<MongoRepo>>().unwrap();
    let room = db
        .find_room(&name)
        .await
        .ok()
        .expect("Failed to fetch room");

    Ok(HttpResponse::Ok().json(room))
}
