use actix_web::{error, get, post, web, HttpRequest, HttpResponse, Responder, Result};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

use crate::repository::mongo_repo::MongoRepo;

const MAX_SIZE: usize = 262_144; // 256k

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub image: String,
    pub email: String,
}

// Register User
#[post("/user/register")]
pub async fn register_user(mut payload: web::Payload, req: HttpRequest) -> Result<impl Responder> {
    let mut body = web::BytesMut::new();

    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let obj = serde_json::from_slice::<User>(&body)?;

    let db = req.app_data::<web::Data<MongoRepo>>().unwrap();
    let exists = db.user_exist(&obj.email).await;

    if exists {
        let _insert = match db.register_user(obj).await {
            Ok(_val) => println!("Successfully inserted!"),
            Err(_) => {}
        };
    }

    Ok(HttpResponse::Accepted().body("User Registered!"))
}

//  Get details from unique 6 digit number
#[get("/user/{id}")]
pub async fn fetch_user_data(id: web::Path<u32>, req: HttpRequest) -> Result<impl Responder> {
    let id = id.into_inner();
    let db = req.app_data::<web::Data<MongoRepo>>().unwrap();
    let obj = db.find_user(id).await.unwrap();

    Ok(HttpResponse::Ok().json(obj))
}

//  Get all 6 digit ids of users in the database
// fix the cursor stream handler
// #[get("/user/all")]
// pub async fn fetch_all_users(req: HttpRequest) -> Result<impl Responder> {
//     let db = req.app_data::<web::Data<MongoRepo>>().unwrap();
//     let users = db.get_all_users().await;

//     Ok(HttpResponse::Ok().json(web::Json(users)))
// }
