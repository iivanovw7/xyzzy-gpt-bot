use crate::server::server;
#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

pub mod commands;
pub mod env;
pub mod handlers;
pub mod menu;
pub mod server;
pub mod types;
pub mod utils;

#[tokio::main]
async fn main() {
    server().await
}
