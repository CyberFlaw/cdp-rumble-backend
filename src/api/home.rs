use actix_web::{get, http::header::ContentType, HttpResponse};

#[get("/")]
pub async fn index_responce() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Server Started... 🚀")
}
