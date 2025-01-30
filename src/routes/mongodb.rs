use actix_web::{web, HttpResponse, Responder};
use mongodb::{Client, options::ClientOptions, Collection};
use mongodb::bson::doc;

async fn get_tokens(client: web::Data<Client>) -> impl Responder {
    let collection: Collection = client.database("your_database").collection("tokens");
    match collection.find(None, None).await {
        Ok(cursor) => {
            let tokens: Vec<_> = cursor.collect().await;
            HttpResponse::Ok().json(tokens)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn add_token(client: web::Data<Client>, token: web::Json<serde_json::Value>) -> impl Responder {
    let collection: Collection = client.database("your_database").collection("tokens");
    match collection.insert_one(token.into_inner(), None).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/mongodb")
            .route("/tokens", web::get().to(get_tokens))
            .route("/tokens", web::post().to(add_token)),
    );
}