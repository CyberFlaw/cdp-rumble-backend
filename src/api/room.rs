use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};

use crate::repository::mongo_repo::MongoRepo;

#[derive(Serialize, Deserialize)]
struct Message {
    msg: String,
    owner: i64,
}

#[derive(Deserialize)]
pub struct AddRoom {
    user: u32,
    friend: u32,
}

// add room to the db
// Querry parametrs should be used instead of explicit routes /join?usr={Userid}&frnd={Userid}
#[post("/join/{user}/{friend}/")]
pub async fn add_room(users: web::Path<AddRoom>, req: HttpRequest) -> Result<impl Responder> {
    format!("Create a room for: {}, {}", users.user, users.friend);

    // db driver code to (insert) create a new record in Rooms collection in Registered db
    let db = req.app_data::<web::Data<MongoRepo>>().unwrap();

    let usr_str = users.user.to_string();
    let frnd_str = users.friend.to_string();

    let exists = db.room_exists(&format!("{}{}", usr_str, frnd_str)).await;
    // return _insert from the if and return it as impl Responder, to give valid HTTP responce
    if exists {
        let _insert = db
            .register_room(users.user, users.friend, format!("{}{}", usr_str, frnd_str))
            .await
            .unwrap();
    }

    // db driver code to (update) push the room id to both users [rooms] attribute

    let responce_str = format!(
        "Room created with room name: {}",
        format!("{}{}", usr_str, frnd_str)
    )
    .to_string();
    Ok(HttpResponse::Ok().body(responce_str))
}

// fectch room details
#[get("/rooms/{name}")]
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
