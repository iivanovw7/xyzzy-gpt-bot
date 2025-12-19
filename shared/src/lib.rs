use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
pub struct LoginResponse {
    pub access_token: String,
    pub user_id: String,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export, export_to = "../generated/bindings.ts")]
pub struct UserResponse {
    pub user_id: String,
}
