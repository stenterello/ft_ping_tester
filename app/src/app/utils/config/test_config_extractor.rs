use serde_json::Value;
use std::fs;
use std::io::{Error, ErrorKind, Result};

pub struct TestConfigExtractor;

impl TestConfigExtractor {
    pub fn decode(file: &str) -> Result<Value> {
        match fs::read_to_string(file) {
            Ok(string) => match serde_json::from_str(&string) {
                Ok(value) => Ok(value),
                Err(_) => Err(Error::new(
                    ErrorKind::InvalidData,
                    "Unable to parse json file.",
                )),
            },
            Err(e) => Err(e),
        }
    }
}
