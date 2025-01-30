use actix_web::{web, App, HttpServer, middleware::Logger};
use std::env;
use mongodb::{Client, options::ClientOptions};

mod auth;
mod chatbot;
mod rpc;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up logging
    std::env::set_var("RUST_LOG", "actix_web=info");
    let _ = env_logger::try_init();

    // Initialize MongoDB client
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let client_options = ClientOptions::parse(&mongo_uri).await.unwrap();
    let mongo_client = Client::with_options(client_options).unwrap();

    // Initialize the web server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(mongo_client.clone()))
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}