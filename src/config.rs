use crate::SsgResult;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub url: String,
    pub title: String,
}

pub fn read_config() -> SsgResult<Config> {
    let file = File::open("website.yaml")?;
    let reader = BufReader::new(file);
    let config: Config = serde_yaml::from_reader(reader)?;
    Ok(config)
}
