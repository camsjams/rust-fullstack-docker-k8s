use crate::car::Car;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::mysql::MySqlPool;

#[get("/cars")]
async fn find_all(db_pool: web::Data<MySqlPool>) -> impl Responder {
    let result = Car::find_all(db_pool.get_ref()).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        _ => HttpResponse::BadRequest().body("Error trying to read all cars from database"),
    }
}

#[get("/car/{id}")]
async fn find(id: web::Path<i32>, db_pool: web::Data<MySqlPool>) -> impl Responder {
    let result = Car::find_by_id(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        _ => HttpResponse::BadRequest().body("Car not found"),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
}
