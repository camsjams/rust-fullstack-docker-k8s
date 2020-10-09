use crate::car::Car;
use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use actix_web::{error, get, web, Error, HttpResponse, Responder};
use redis_async::{resp::RespValue, resp_array};
use sqlx::mysql::MySqlPool;

#[get("/cars")]
async fn find_all(
    db_pool: web::Data<MySqlPool>,
    redis: web::Data<Addr<RedisActor>>,
) -> impl Responder {
    let existing_cache = redis.send(Command(resp_array!["GET", "cars:all"])).await;
    match existing_cache {
        Err(e) => Err(Error::from(e)),
        Ok(res) => match res {
            Ok(val) => {
                match val {
                    RespValue::Error(err) => {
                        return Err(error::ErrorInternalServerError(err));
                    }
                    RespValue::SimpleString(s) => {
                        if let Ok(val) = serde_json::from_str(&s) {
                            return Ok(Some((val, value)));
                        }
                    }
                    RespValue::BulkString(s) => {
                        if let Ok(val) = serde_json::from_slice(&s) {
                            return Ok(Some((val, value)));
                        }
                    }
                    _ => (),
                }
                Ok(None) =>
            }
            Err(err) => Err(error::ErrorInternalServerError(err)),
        },
    };

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
