use actix_web::{get, http::header::ContentType, HttpResponse};

#[get("/")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Server Started... 🚀")
}
