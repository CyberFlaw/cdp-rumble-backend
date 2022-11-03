use actix_web::{error, post, web, HttpResponse, Result};
use futures::StreamExt;
use serde::Deserialize;

const MAX_SIZE: usize = 262_144; // 256k

#[derive(Deserialize)]
struct UserForm {
    name: String,
    image: String,
    email: String,
}

#[post("/user/register")]
pub async fn register_user(mut payload: web::Payload) -> Result<HttpResponse, error::Error> {
    let mut body = web::BytesMut::new();

    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let obj = serde_json::from_slice::<UserForm>(&body)?;
    println!("{} {} {}", obj.name, obj.email, obj.image);

    //     add db code
    Ok(HttpResponse::Ok().body("Successfully registered the user!"))
}
