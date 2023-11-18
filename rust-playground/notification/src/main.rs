extern crate log;

mod application;

use std::env;

use log::info;
use dotenv::dotenv;
use rustyttp::http::structs::ConfigBuilder;

use crate::application::routes;


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
    
    rustyttp::http::connection_handling::open_connection(Some(configuration), routes::route_service::execute_request);
    info!("Shutting down.");
}
