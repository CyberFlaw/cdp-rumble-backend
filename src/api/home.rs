use actix_web::{
    //     error::ResponseError,
    get,
    http::header::ContentType,
    //     web::Path,
    //     HttpResponse,
    //     http::{header::ContentType, StatusCode},
    //     post, put,
    //     web::Data,
    // web::Json,
    HttpResponse,
};

// use serde::{Deserialize, Serialize};

// use derive_more::Display;

// pub struct TaskIdentifier {
//       task_global_id: String
// }

// #[get("/")]
// pub async fn get_task() -> Json<String> {
//     return Json("Hello World".to_string());
// }

#[get("/")]
pub async fn index_responce() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Server Started... 🚀")
}
