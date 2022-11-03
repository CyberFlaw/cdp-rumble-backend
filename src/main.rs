mod api;
mod model;
mod repository;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::home::index_responce;
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use repository::mongodb_repo::MongoRepo;

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

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(db_data.clone())
            .service(index_responce)
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
