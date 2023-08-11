mod rappels;

use crate::database_service;
use postgres::Error;

pub use rappels::Rappel;



pub fn get_rappels() -> String {
    let rows = match get_all() {
        Ok(results) => results,
        Err(error) => panic!("Fatal: {}", error),
    };
    return format!("{{\"result\":[{}]}}", rows.join(", "));
}


pub fn get_all() -> Result<Vec<String>, Error> {
    let mut client = database_service::connect()?;

    let mut all_rows: Vec<String> = Vec::new();

    println!("Query on table from database_service");

    match client.query("SELECT * from rappels", &[]) {
        Ok(rows) => {
            for row in rows {
                let name: String = row.get(0);
                all_rows.push(format!("{{\"id\": \"{}\"}}", name));
            }
        }
        Err(error) => println!("{}", error),
    }

    client.close()?;

    return Ok(all_rows);
}

pub fn new_rappel(rappel : Rappel) -> String {
    return match add_one(rappel) {
        Ok(result)  => result.to_string(),
        Err(error) => panic!("Fatal: {}", error)
    };
}

pub fn add_one(rappel : Rappel) -> Result<u64, Error> {
    let mut client = database_service::connect()?;

    println!("Query on table from database_service");

    let row_update = client.execute("INSERT INTO rappels VALUES ($1, TO_DATE($2, 'YYYY-MM-DD'), $3, $4)", &[&rappel.nom, &rappel.date_limite, &rappel.repetition, &rappel.criticite])?;

    client.close()?;

    return Ok(row_update);
}
