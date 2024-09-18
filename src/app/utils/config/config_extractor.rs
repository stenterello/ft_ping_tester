use serde_derive::Deserialize;
use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Config {
    pub locations: Locations,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Locations {
    pub ft_ping_dir: String,
    pub ft_ping_name: String,
    pub ping_dir: String,
    pub ping_name: String,
    pub test_conf_path: String,
}

#[derive(Debug, Default)]
pub struct ConfigExtractor;

impl ConfigExtractor {
    pub fn decode(file: &str) -> Result<Config> {
        let contents: String = match fs::read_to_string(file) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        let conf: Config = match toml::from_str(&contents) {
            Ok(values) => values,
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Unable to parse toml file.",
                ))
            }
        };

        if !Path::new(conf.locations.ft_ping_dir.as_str()).exists()
            || !Path::new(conf.locations.ping_dir.as_str()).exists()
            || !Path::new(conf.locations.test_conf_path.as_str()).exists()
        {
            Err(Error::new(ErrorKind::NotFound, "Wrong paths in conf.toml"))
        } else if !conf.locations.ft_ping_dir.ends_with('/')
            || !conf.locations.ping_dir.ends_with('/')
        {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Directory paths must end with '/'. Change conf.toml",
            ))
        } else {
            Ok(conf)
        }
    }
}
