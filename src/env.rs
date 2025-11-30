use dotenv::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Env {
    pub token: String,
    pub open_api_key: String,
    pub user_id: u64,
    pub model: String,
    pub database_url: String,
}

lazy_static! {
    pub static ref ENV: Env = get_env();
}

fn get_env() -> Env {
    dotenv().ok();

    match envy::from_env::<Env>() {
        Ok(env) => env,
        Err(error) => panic!("Env configuration Error: {:#?}", error),
    }
}
