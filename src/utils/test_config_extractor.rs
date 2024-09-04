use serde_json::{Value};
use std::fs;

#[derive(Debug, Default)]
pub struct TestConfigExtractor;

impl TestConfigExtractor {
    pub fn decode(file: String) -> Value {
        let contents = fs::read_to_string(file).expect("Unable to read tests file");
        serde_json::from_str(contents.as_str()).unwrap()
    }
}
