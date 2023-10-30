extern crate log;

mod application;

use log::info;
use dotenv::dotenv;

fn main() {
    info!("Starting server");

    info!("Initializing Environment variables");
    dotenv().ok();
 
    info!("Initializing Logger");
    env_logger::init();
    
    application::http::connection_handling::open_connection();
    info!("Shutting down.");
}
