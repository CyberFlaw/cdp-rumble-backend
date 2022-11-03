mod api;
mod model;
mod repository;

use actix_web::{
    error,
    middleware::Logger,
    web::{Data, JsonConfig},
    App, HttpResponse, HttpServer,
};
use api::home;
use api::user;
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
// use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const HOST: &str = "127.0.0.1";
    const PORT: u16 = 5000;

    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("./keys/key.pem", SslFiletype::PEM)
    //     .unwrap();
    // builder
    //     .set_certificate_chain_file("./keys/cert.pem")
    //     .unwrap();

    // let db = MongoRepo::init().await;
    // let db_data = Data::new(db);

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        let json_config = JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });
        App::new()
            .wrap(logger)
            // .app_data(db_data.clone())
            .app_data(json_config)
            .service(home::index_responce)
            .service(home::json_check)
            .service(user::register_user)
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
