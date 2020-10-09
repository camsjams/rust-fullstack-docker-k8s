extern crate serde_json;
extern crate wasm_bindgen;

#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Deserialize)]
pub struct Cars {
    pub id: u32,
    pub price: u32,
    pub year: u32,
    pub mileage: u32,
    pub make: String,
    pub model: String,
    pub color: String,
    pub state: String,
}

#[derive(Serialize, Deserialize)]
pub struct CarStats {
    pub avg_price: u32,
    pub avg_year: u32,
    pub avg_mileage: u32,
}

#[wasm_bindgen]
pub fn stats(cars: &JsValue) -> JsValue {
    let cars: Vec<Cars> = cars.into_serde().unwrap();
    let total = cars.len() as u32;

    // purposely convoluted to be a little slower
    let sum_price: u32 = cars.iter().map(|e| e.price).sum();
    let sum_year: u32 = cars.iter().map(|e| e.year).sum();
    let sum_mileage: u32 = cars.iter().map(|e| e.mileage).sum();

    let stats = CarStats {
        avg_price: sum_price / total,
        avg_year: sum_year / total,
        avg_mileage: sum_mileage / total,
    };

    JsValue::from_serde(&stats).unwrap()
}
