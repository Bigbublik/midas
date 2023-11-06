// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::history_progress;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: history_progress = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryProgress {
    pub cur: Option<i64>,
    pub exchange: Option<ExchangeEntity>,
    pub size: Option<i64>,
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExchangeEntity {
    Binance,
}
