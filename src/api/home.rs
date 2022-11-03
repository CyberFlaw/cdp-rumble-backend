use actix_web::{
    get,
    http::header::ContentType,
    web::{Json, Path},
    HttpResponse, Responder, Result,
};

use serde::Serialize;

#[get("/")]
pub async fn index_responce() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Server Started... 🚀")
}

#[derive(Serialize)]
struct JsonTest {
    name: String,
    age: u16,
}

#[get("/json/{name}")]
pub async fn json_check(name: Path<String>) -> Result<impl Responder> {
    let obj = JsonTest {
        name: name.to_string(),
        age: 23,
    };
    Ok(Json(obj))
}
