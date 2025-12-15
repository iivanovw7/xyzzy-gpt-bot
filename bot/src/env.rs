use dotenv::dotenv;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Env {
    pub token: String,
    pub open_api_key: String,
    pub user_id: u64,
    pub database_url: String,
    pub jwt_secret: String,
    pub web_app_path: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_env() {
        let env = get_env();
        assert_ne!(env.token, "".to_string());
    }

    #[test]
    fn it_gets_env_from_the_lazy_static() {
        let env = &ENV;
        assert_ne!(env.token, "".to_string());
    }
}
