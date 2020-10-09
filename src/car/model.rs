use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use serde::Serialize;
use sqlx::{FromRow, MySqlPool};
use std::future::{ready, Ready};

#[derive(Serialize, FromRow)]
pub struct Car {
    pub id: u32,
    pub price: u32,
    pub year: u32,
    pub mileage: u32,
    pub make: String,
    pub model: String,
    pub color: String,
    pub state: String,
}

impl Responder for Car {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

impl Car {
    pub async fn find_all(pool: &MySqlPool) -> Result<Vec<Car>> {
        let mut cars = vec![];
        let recs = sqlx::query!(
            r#"
                SELECT id, price, year, mileage, make, model, color, state
                    FROM car
                ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            cars.push(Car {
                id: rec.id,
                price: rec.price,
                year: rec.year,
                mileage: rec.mileage,
                make: rec.make,
                model: rec.model,
                color: rec.color,
                state: rec.state,
            });
        }

        Ok(cars)
    }

    pub async fn find_by_id(id: i32, pool: &MySqlPool) -> Result<Car> {
        let rec = sqlx::query!(
            r#"
                SELECT id, price, year, mileage, make, model, color, state
                    FROM car
                WHERE id = ?
                "#,
            id
        )
        .fetch_one(&*pool)
        .await?;

        Ok(Car {
            id: rec.id,
            price: rec.price,
            year: rec.year,
            mileage: rec.mileage,
            make: rec.make,
            model: rec.model,
            color: rec.color,
            state: rec.state,
        })
    }
}
