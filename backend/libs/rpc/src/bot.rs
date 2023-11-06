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
use std::collections::HashMap;

pub type Bot = HashMap<String, Option<serde_json::Value>>;
