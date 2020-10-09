#![feature(future_readiness_fns)]
#[macro_use]
extern crate log;

use actix_files::Files;
use actix_redis::RedisActor;
use actix_web::{middleware, App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;
use meilisearch_sdk::client::*;
use sqlx::mysql::MySqlPool;
use std::env;

mod car;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL is not set in .env file");
    let db_pool = MySqlPool::new(&database_url).await?;
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");

    let client = Client::new("http://localhost:7700", "masterKey");
    info!("Indexing Search");
    car::fixtures(client, &db_pool.clone()).await?;
    info!("Search Ready");

    let server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .data(RedisActor::start(redis_url.clone()))
            .wrap(middleware::Logger::default())
            .configure(car::init)
            .service(Files::new("/", "./app/dist/public/").index_file("index.html"))
    });

    info!("Starting server");
    server.bind(format!("{}:{}", host, port))?.run().await?;

    Ok(())
}
