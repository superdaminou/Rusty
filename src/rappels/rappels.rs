pub fn get_all() -> Result<Vec<String>, Error> {
    let mut client = connect()?;

    let mut all_rows : Vec<String> = Vec::new();

    println!("Query on table from database_service");

    match client.query ("SELECT * from rappels", &[]) {
        Ok(rows) => {
            for row in rows {
                let name: String =  row.get (0);
                all_rows.push (format!("{{\"id\": \"{}\"}}", name));
            }
        },
        Err(error) => println!("{}", error)
    }

    client.close()?;

    return Ok(all_rows);

}