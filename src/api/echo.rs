use crate::repository::mongo_repo::MongoRepo;
use crate::websocket::{lobby::Lobby, ws::WsConn};

use actix::Addr;
use actix_web::{delete, error, get, post, put, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use bson::oid::ObjectId;
use futures::StreamExt;
use serde::{Deserialize, Serialize};

const MAX_SIZE: usize = 262_144; // 256k

#[get("/echo/{room_id}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: web::Payload,
    room_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let srv = req
        .app_data::<web::Data<Addr<Lobby>>>()
        .unwrap()
        .get_ref()
        .clone();

    let room_id = room_id.into_inner();

    let ws = WsConn::new(room_id, srv);

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}

/*
-----------------------------------------------------
Note the code below is not tested!
-----------------------------------------------------
*/

// sync the messgae to database
#[derive(Serialize, Deserialize)]
struct Message {
    owner: u32,
    text: String,
}

#[post("/sync/{room_id}")]
pub async fn sync_messages(
    mut payload: web::Payload,
    room_id: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let mut body = web::BytesMut::new();

    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("Buffer Overflow!"));
        }
        body.extend_from_slice(&chunk);
    }

    let ob = serde_json::from_slice::<Message>(&body)?;

    let room_id = room_id.into_inner();
    let db = req.app_data::<web::Data<MongoRepo>>().unwrap();

    db.add_message(room_id.clone(), ob.text.clone(), ob.owner)
        .await;

    Ok(HttpResponse::Accepted().body("Database synched!"))
}

/*
------------------------------------------------------------
Get all messages from database : Improvements required

The below code simple fetches all the messages of the user
after they are logged in (endpoint request will be send
after auth). This cause massive network bandwidth usage due
to a large ammount of data being downloaded from the
database to the server and then to the client. Its best to
keep a local archive backup in the front end: a backup with
like 10 to 20 messages of every chat for user. If a user
request additional message (by scroll event or some other
method), then only server have to fetch extra message from
server. This should be atmost 50 documents at a time. A
timestamp would be required to execute such architecture.
------------------------------------------------------------
*/
#[get("/sync/all/{room_id}")]
pub async fn get_all_messages(
    room_id: web::Path<String>,
    req: HttpRequest,
) -> error::Result<impl Responder> {
    let db = req.app_data::<web::Data<MongoRepo>>().unwrap();
    let msgs = db.get_all_messages(room_id.clone()).await;

    Ok(HttpResponse::Ok().json(web::Json(msgs)))
}

// edit a message (query should be used <room_id & msg_obj_id & text>)
#[derive(Deserialize)]
pub struct EditQuery {
    room_id: String,
    msg_id: ObjectId,
    text: String,
}

#[put("/sync/msg")]
pub async fn edit_message(
    info: web::Query<EditQuery>,
    req: HttpRequest,
) -> error::Result<impl Responder> {
    let db = req.app_data::<web::Data<MongoRepo>>().unwrap();
    db.update_message(info.room_id.clone(), info.msg_id, info.text.clone())
        .await;

    let responce_str = format!("Updated Message to: {}", info.text).to_string();

    Ok(HttpResponse::Ok().body(responce_str))
}

// delete a message (query should be used <room_id & msg_obj_id>)
#[derive(Deserialize)]
pub struct DeleteQuery {
    room_id: String,
    msg_id: ObjectId,
}

#[delete("/sync/msg")]
pub async fn delete_message(
    info: web::Query<DeleteQuery>,
    req: HttpRequest,
) -> error::Result<impl Responder> {
    let db = req.app_data::<web::Data<MongoRepo>>().unwrap();
    db.delete_message(info.room_id.clone(), info.msg_id).await;

    let responce_str = format!("Message deleted").to_string();

    Ok(HttpResponse::Ok().body(responce_str))
}
