use postgres::{Client, Error, NoTls};

pub fn connect() -> Result<Client, Error> {
    println!("Connection to database");
    // Database client creation
    let client = Client::connect(
        "postgresql://postgres:john@localhost:5432/notification_playground",
        NoTls,
    )?;

    Ok(client)
}
