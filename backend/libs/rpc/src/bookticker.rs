// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::bookticker;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: bookticker = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bookticker {
  pub ask_price: Option<String>,
  pub ask_qty: Option<String>,
  pub bid_price: Option<String>,
  pub bid_qty: Option<String>,
  pub id: Option<String>,
  pub symbol: Option<String>,
}
