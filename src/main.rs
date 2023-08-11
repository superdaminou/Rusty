extern crate log;

mod application;

use log::info;
use dotenv::dotenv;

// Listen on port 7878 and for each stream incoming create a new thread.
fn main() {
    info!("Starting server");

    info!("Initializing");
    dotenv().ok();
    env_logger::init();
    
    application::utils::connection_handling::open_connection();
    info!("Shutting down.");
}
