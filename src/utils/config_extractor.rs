use serde_derive::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    locations: Locations,
}

#[derive(Debug, Deserialize)]
pub struct Locations {
    pub ft_ping_dir: String,
    pub ping_dir: String,
    pub test_conf_path: String,
}

#[derive(Debug, Default)]
pub struct ConfigExtractor;

impl ConfigExtractor {
    pub fn decode(file: String) -> Locations {
        let contents = fs::read_to_string(file).expect("Unable to read config file");
        let conf: Config = toml::from_str(&contents).unwrap();

        conf.locations
    }
}
