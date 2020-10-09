use crate::car::Car;
use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use actix_web::{get, web, HttpResponse, Responder};
use redis_async::{resp::RespValue, resp_array};
use sqlx::mysql::MySqlPool;

#[get("/cars")]
async fn find_all(
    db_pool: web::Data<MySqlPool>,
    redis: web::Data<Addr<RedisActor>>,
) -> impl Responder {
    let res = redis
        .send(Command(resp_array!["GET", "cars:all"]))
        .await
        .unwrap();

    let existing_cache: Option<Vec<Car>> = match res {
        Ok(RespValue::SimpleString(s)) => {
            println!("SimpleString response");
            if let Ok(val) = serde_json::from_str(&s) {
                Some(val)
            } else {
                None
            }
        }
        Ok(RespValue::BulkString(s)) => {
            println!("BulkString response");
            if let Ok(val) = serde_json::from_slice(&s) {
                Some(val)
            } else {
                None
            }
        }
        _ => None,
    };

    match existing_cache {
        // cache found
        Some(cars) => {
            info!("Using Cars from Cache");
            HttpResponse::Ok().json(cars)
        }
        // No cache
        None => {
            info!("Using Cars from Database");
            let result = Car::find_all(db_pool.get_ref()).await;
            match result {
                Ok(cars) => {
                    let redis_string = serde_json::to_string(&cars).unwrap();
                    let save_result = redis
                        .send(Command(resp_array![
                            "SET",
                            "cars:all",
                            redis_string,
                            "EX",
                            "300"
                        ]))
                        .await;

                    match save_result {
                        _ => HttpResponse::Ok().json(cars),
                    }
                }

                Err(_) => HttpResponse::BadRequest().body("Cars not found"),
            }
        }
    }
}

#[get("/car/{id}")]
async fn find(id: web::Path<i32>, db_pool: web::Data<MySqlPool>) -> impl Responder {
    let result = Car::find_by_id(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(car) => HttpResponse::Ok().json(car),
        _ => HttpResponse::BadRequest().body("Car not found"),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
}
