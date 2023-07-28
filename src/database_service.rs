use postgres::{Client, Error, NoTls};

pub fn connect() ->  Result<(), Error> {
    
    println!("Connection to database");
    // Database client creation
    let mut client = Client::connect("postgresql://postgres:john@localhost:5432/notification_playground", NoTls)?;

    println!("Query on table");
    for row in client.query ("SELECT * from rappels", &[])? {
        let id : String = row.get (0);
        println!("{}", id)
    }

    Ok(())
}
