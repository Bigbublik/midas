// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::api_key;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: api_key = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiKey {
    pub exchange: Option<ExchangeEntity>,
    pub id: Option<String>,
    pub label: Option<String>,
    pub prv: Option<String>,
    #[serde(rename = "pub")]
    pub api_key_pub: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExchangeEntity {
    Binance,
}
