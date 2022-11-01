use actix_web::{
    //     error::ResponseError,
    get,
    //     http::{header::ContentType, StatusCode},
    //     post, put,
    //     web::Data,
    web::Json,
    //     web::Path,
    //     HttpResponse,
};

// use serde::{Deserialize, Serialize};

// use derive_more::Display;

// pub struct TaskIdentifier {
//       task_global_id: String
// }

#[get("/")]
pub async fn get_task() -> Json<String> {
    return Json("Hello World".to_string());
}
