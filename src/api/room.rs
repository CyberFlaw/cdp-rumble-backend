use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, Result};
use serde::Deserialize;

use crate::repository::mongo_repo::MongoRepo;

#[derive(Deserialize)]
pub struct AddQuery {
    user: u32,
    friend: u32,
}

// add room to the db
#[post("/join")]
pub async fn add_room(info: web::Query<AddQuery>, req: HttpRequest) -> Result<impl Responder> {
    let user_str = info.user.to_string();
    let friend_str = info.friend.to_string();
    let room_id = format!("{}{}", user_str, friend_str);

    let db = req.app_data::<web::Data<MongoRepo>>().unwrap();
    let exists = db.room_exists(&room_id).await;

    if exists {
        let _insert = db
            .register_room(info.user, info.friend, room_id.clone())
            .await
            .unwrap();
    }

    // db.add_chat_room(room_id);

    // db driver code to (update) push the room id to both users [room] attribute
    db.append_room_user(info.user, format!("{}{}", user_str, friend_str).to_string())
        .await;
    db.append_room_user(
        info.friend,
        format!("{}{}", user_str, friend_str).to_string(),
    )
    .await;

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
