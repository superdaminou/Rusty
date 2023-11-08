extern crate log;

mod application;

use log::info;
use dotenv::dotenv;

use crate::application::{http, routes};

fn main() {
    info!("Starting server");

    info!("Initializing Environment variables");
    dotenv().ok();
 
    info!("Initializing Logger");
    env_logger::init();
    
    http::connection_handling::open_connection(routes::route_service::execute_request);
    info!("Shutting down.");
}
