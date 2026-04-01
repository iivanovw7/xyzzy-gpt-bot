use anyhow::Result;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub open_ai: OpenAiConfig,
    pub web: WebConfig,
    pub api: ApiConfig,
    pub surf: SurfConfig,
    pub budgeting: BudgetingConfig,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OpenAiConfig {
    pub model: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BudgetingConfig {
    pub recurrent_income_categories: Vec<i64>,
    pub recurrent_payment_categories: Vec<i64>,
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SurfConfig {
    pub location: String,
    pub lon: f32,
    pub lat: f32,
}

lazy_static! {
    pub static ref CONFIG: Config = get_config().expect("Failed to load configuration");
}

fn get_config() -> Result<Config> {
    let file = fs::read_to_string("config.toml").expect("Unable to read config.toml");
    let config: Config = toml::from_str(&file).expect("Unable to parse config.toml");

    Ok(config)
}
