mod api;
mod model;
mod repository;

use actix_web::{error, middleware::Logger, web, App, HttpResponse, HttpServer};
use api::{home, room, user};
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use repository::mongo_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const HOST: &str = "127.0.0.1";
    const PORT: u16 = 5000;

    // making an ssl certificate for the server
    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("./keys/key.pem", SslFiletype::PEM)
    //     .unwrap();
    // builder
    //     .set_certificate_chain_file("./keys/cert.pem")
    //     .unwrap();

    let db = MongoRepo::init().await;
    let db_data = web::Data::new(db);

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });
        App::new()
            .wrap(logger)
            .app_data(db_data.clone())
            .app_data(json_config)
            .service(home::index_responce) // method: post, route: "/"
            .service(user::register_user) // method: get, route: "/user/register""
            .service(user::fetch_user_data) // method: get, route: "/user/{id}"
            .service(user::fetch_all_users) // method: get, route: "/users/all"
            .service(room::add_room) // method: post, route: "/join/{user}/{friend}/"
            .service(room::fetch_room_data) // method: get, route: "/rooms/{name}"
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
