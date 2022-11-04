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

    //  update database
    let db = req.app_data::<web::Data<MongoRepo>>().unwrap();
    let _insert = match db.register_user(obj).await {
        Ok(_val) => println!("Successfully inserted!"),
        Err(_) => {}
    };

    Ok(HttpResponse::Accepted().body("User Registered!"))
}

//  Get details from unique 6 digit number

#[get("/user/{id}")]
pub async fn fetch_user_data(id: web::Path<String>) -> Result<impl Responder> {
    println!("{id}");
    // fetch data from db and populate struct
    let obj = User {
        name: "hello".to_string(),
        email: "test@boi.com".to_string(),
        image: "https:img.com".to_string(),
    };
    Ok(web::Json(obj))
}

//  Get all 6 digit ids of users in the database
#[get("/user/all")]
pub async fn fetch_all_users() -> Result<impl Responder> {
    //  make vec mutable before updating code
    let user_list: Vec<i64> = vec![0, 1, 2];

    //  fetch db and push_back to vector

    Ok(HttpResponse::Ok().json(user_list))
}
