use actix_web::{web, App, HttpServer, middleware::Logger};
use std::env;

mod auth;
mod chatbot;
mod rpc;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up logging
    std::env::set_var("RUST_LOG", "actix_web=info");
    let _ = env_logger::try_init();

    // Initialize the web server
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}