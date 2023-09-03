use crate::application::rappels::structures::Rappel;
use crate::application::database::database_service;
use postgres::Error;
use log::info;


pub fn add_one(rappel : Rappel) -> Result<u64, Error> {
    let mut client = database_service::connect()?;

    info!("Query on table from database_service");

    let row_update = client.execute("INSERT INTO rappels VALUES ($1, TO_DATE($2, 'YYYY-MM-DD'), $3, $4)", &[&rappel.nom, &rappel.date_limite, &rappel.repetition, &rappel.criticite])?;

    client.close()?;

    Ok(row_update)
}


pub fn get_all() -> Result<Vec<Rappel>, Error> {
    let mut client = database_service::connect()?;
    info!("Query on table from database_service");

    let rappels : Vec<Rappel> = match client.query("SELECT * from rappels", &[]) {

        Ok(rows) =>  
            rows.iter().map(|row|Rappel {
                    nom: row.get(1),
                    date_limite:  row.get("date_limite"),
                    repetition: row.get("repetition"),
                    criticite: row.get("criticite")
            }).collect(),
        Err(error) => {
            client.close()?;
            return Err(error)
        },
    };

    client.close()?;

    info!("Succesful query");

    Ok(rappels)
}


pub fn get_one(id: i32) -> Result<Vec<Rappel>, Error> {
    let mut client = database_service::connect()?;
    info!("Getting {}", id);

    let rappels : Vec<Rappel> = match client.query("SELECT * from rappels WHERE rappel_id=$1", &[&id]) {

        Ok(rows) =>  
            rows.iter().map(|row|Rappel {
                    nom: row.get(1),
                    date_limite:  row.get("date_limite"),
                    repetition: row.get("repetition"),
                    criticite: row.get("criticite")
            }).collect(),
        Err(error) => {
            client.close()?;
            return Err(error)
        },
    };

    client.close()?;

    info!("Succesful query");

    Ok(rappels)
}