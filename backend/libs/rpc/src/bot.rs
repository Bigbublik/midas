// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::bot;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: bot = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bot {
    pub base_currency: Option<String>,
    pub condition: Option<String>,
    pub created_at: Option<TimestampSchema>,
    pub exchange: Option<ExchangeEntity>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub trading_amount: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimestampSchema {
    pub nanos: Option<i64>,
    pub seconds: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExchangeEntity {
    Binance,
}
