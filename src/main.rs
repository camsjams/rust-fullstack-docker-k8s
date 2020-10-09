#![feature(future_readiness_fns)]
#[macro_use]
extern crate log;

use actix_files::Files;
use actix_web::{App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::env;

mod car;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = MySqlPool::new(&database_url).await?;
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");

    let server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .configure(car::init)
            .service(Files::new("/", "./app/dist/").index_file("index.html"))
    });

    info!("Starting server");
    server.bind(format!("{}:{}", host, port))?.run().await?;

    Ok(())
}
