use serde_derive::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Default)]
pub struct Config {
    pub config: Option<ConfigValues>,
    pub valid: bool,
}

#[derive(Debug, Deserialize)]
pub struct ConfigValues {
    pub locations: Locations,
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
    pub fn decode(file: String) -> Config {
        let contents = fs::read_to_string(file).expect("Unable to read config file");
        let conf_values: ConfigValues = toml::from_str(&contents).unwrap();

        let mut conf: Config = Config::default();
        if !Path::new(conf_values.locations.ft_ping_dir.as_str()).exists()
            || !Path::new(conf_values.locations.ping_dir.as_str()).exists()
            || !Path::new(conf_values.locations.test_conf_path.as_str()).exists()
        {
            eprintln!("Wrong paths in conf.toml");
            conf.valid = false;
        } else {
            conf.valid = true;
            conf.config = Some(conf_values);
        }

        conf
    }
}
