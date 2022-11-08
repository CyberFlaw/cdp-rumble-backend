mod api;
mod model;
mod repository;
mod websocket;

use actix::Actor;
use actix_web::{error, middleware::Logger, web, App, HttpResponse, HttpServer};
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use api::{echo, home, room, user};
use repository::mongo_repo::MongoRepo;
use websocket::lobby;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const HOST: &str = "0.0.0.0";
    const PORT: u16 = 5000;

    // making an ssl certificate for the server
    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("./keys/key.pem", SslFiletype::PEM)
    //     .unwrap();
    // builder
    //     .set_certificate_chain_file("./keys/cert.pem")
    //     .unwrap();

    let chat_server = lobby::Lobby::default().start();
    let chat_server_data = web::Data::new(chat_server);

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
            .app_data(json_config)
            .app_data(db_data.clone())
            .app_data(chat_server_data.clone())
            .service(home::health_check) // method: post, route: "/"
            .service(user::register_user) // method: get, route: "/user/register""
            .service(user::fetch_user_data) // method: get, route: "/user/{id}"
            .service(user::fetch_all_users) // method: get, route: "/users/all"
            .service(room::add_room) // method: post, route: "/join?user={user}&friend={friend}"
            .service(room::fetch_room_data) // method: get, route: "/rooms/{name}"
            .service(echo::sync_messages) // method: post, route: "/sync/{room_id}""
            .service(echo::get_all_messages) // method: get, route: "/sync/all/{room_id}"
            .service(echo::edit_message) // method: put, route: "/sync/msg?room_id={room_id}&msg_id={msg_id}&text={new_msg}"
            .service(echo::delete_message) // method: delete, route: "/sync/msg?room_id={room_id}&msg_id={msg_id}"
            .service(echo::start_connection) // wscat localhost:5000/echo/{room_id}
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
