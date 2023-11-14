use postgres::{Client, Error, NoTls};
use std::env;
use log::{warn, error};

const DEFAULT_DATABASE : &str = "postgresql://postgres:playground@localhost:5432/notification_playground";

pub fn connect() -> Result<Client, Error> {
    println!("Connection to database");
    
    let database = match env::var("DATABASE") {
        Ok(database) => database,
        Err(error) => {
            error!("{}", error);
            warn!("Couldn't retrieve database env, swiching to default: {}", DEFAULT_DATABASE);
            DEFAULT_DATABASE.to_string()
        }
    };
    
    // Database client creation
    let client = Client::connect(
        &database,
        NoTls,
    )?;

    Ok(client)
}
