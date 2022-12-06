use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod search;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = &env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = &env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("{}:{}", host, port);

    HttpServer::new(|| App::new().route("/", web::post().to(search::viewer::find_similar_words)))
        .bind(address)?
        .run()
        .await
}
