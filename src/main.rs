#[macro_use]
extern crate log;

use actix_files::Files;
use actix_redis::RedisActor;
use actix_web::{middleware, web, App, HttpRequest, HttpServer, Responder};
use anyhow::Result;
use dotenv::dotenv;
use meilisearch_sdk::client::*;
use sqlx::mysql::MySqlPool;
use std::env;

mod car;

async fn ping(_req: HttpRequest) -> impl Responder {
    format!(
        "I am healthy: {} v{}",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION")
    )
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL is not set");
    let host = env::var("HOST").expect("HOST is not set");
    let port = env::var("PORT").expect("PORT is not set");
    let meilisearch_url = env::var("MEILISEARCH_URL").expect("MEILISEARCH_URL is not set");
    let db_pool = MySqlPool::new(&database_url).await?;
    let client = Client::new(&meilisearch_url, "masterKey");
    info!("Indexing Search");
    car::fixtures(client, &db_pool.clone()).await?;
    info!("Search Ready");

    let server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .data(RedisActor::start(redis_url.clone()))
            .wrap(middleware::Logger::default())
            .configure(car::init)
            .route("/ping", web::get().to(ping))
            .service(Files::new("/", "./app/dist/public/").index_file("index.html"))
    });

    info!("Starting server");
    server.bind(format!("{}:{}", host, port))?.run().await?;

    Ok(())
}
