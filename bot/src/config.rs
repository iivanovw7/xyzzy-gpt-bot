use anyhow::Result;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub open_ai: OpenAiConfig,
    pub web: WebConfig,
    pub api: ApiConfig,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OpenAiConfig {
    pub model: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ApiConfig {
    pub port: u16,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WebConfig {
    pub auth: bool,
    pub url: String,
    pub port: u16,
    pub dist: String,
}

lazy_static! {
    pub static ref CONFIG: Config = get_config().expect("Failed to load configuration");
}

fn get_config() -> Result<Config> {
    let file = fs::read_to_string("config.toml").expect("Unable to read config.toml");
    let config: Config = toml::from_str(&file).expect("Unable to parse config.toml");

    Ok(config)
}
