extern crate log;

mod application;

use std::env;

use log::info;
use dotenv::dotenv;
use crate::application::routes;
use rustyttp::{Routes, ConfigBuilder};
use rustyttp::open_connection;

fn main() {
    info!("Starting server");

    info!("Initializing Environment variables");
    dotenv().ok();
 
    info!("Initializing Logger");
    env_logger::init();

    let configuration = ConfigBuilder::new()
        .adress(env::var("SERVER_ADRESS")
            .unwrap_or("127.0.0.1".to_string()))
        .port(8080)
        .build();
    
    open_connection(Some(configuration), Routes::from(routes()));

    info!("Shutting down.");
}
