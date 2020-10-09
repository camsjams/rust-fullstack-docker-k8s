use crate::car::Car;
use anyhow::Result;
use meilisearch_sdk::{client::*, document::*};
use sqlx::mysql::MySqlPool;

// That trait is required to make a struct usable by an index
impl Document for Car {
    type UIDType = u32;

    fn get_uid(&self) -> &Self::UIDType {
        &self.id
    }
}

pub async fn fixtures(client: Client<'_>, pool: &MySqlPool) -> Result<bool> {
    let index = client.get_or_create("cars").await.unwrap();
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

    index.add_documents(&cars, Some("id")).await.unwrap();

    Ok(true)
}
