use crate::application::{errors::TechnicalError, rappels::structs::Rappel};
use crate::application::database::database_service;
use postgres::Error;
use log::info;

pub fn add_one(rappel : Rappel) -> Result<u64, Error> {
    let mut client = database_service::connect()?;
    info!("Adding one rappel");
    let row_update = client.execute("INSERT INTO rappels (nom, date_limite, repetition, criticite) VALUES ($1, $2, $3, $4)", &[&rappel.nom, &rappel.date_limite, &rappel.repetition, &rappel.criticite])?;
    client.close()?;
    Ok(row_update)
}


pub fn get_all() -> Result<Vec<Rappel>, Error> {
    let mut client = database_service::connect()?;
    info!("Getting all rappels");

    let rappels : Vec<Rappel> = match client.query("SELECT * from rappels", &[]) {
        Ok(rows) =>  
            rows.iter().map(|row| Rappel::from(row)).collect(),
        Err(error) => {
            client.close()?;
            return Err(error)
        },
    };

    client.close()?;

    info!("Succesful query");

    Ok(rappels)
}


pub fn get_one(id: i32) -> Result<Rappel, TechnicalError> {
    let mut client = database_service::connect()?;
    info!("Getting rappel: {}", id);

    let rappels : Vec<Rappel> = match client.query("SELECT * from rappels WHERE rappel_id=$1", &[&id]) {

        Ok(rows) =>  
            rows.iter().map(|row| Rappel::from(row)).collect(),
        Err(error) => {
            client.close()?;
            return Err(TechnicalError::from(error))
        },
    };

    client.close()?;

    return match rappels.iter().count() {
        0  => Err(TechnicalError::from("No rappel")),
        1 => Ok(rappels.iter().clone().next().unwrap().clone()),
        _ => Err(TechnicalError::from("Should have 1 result max"))
    };
}


pub fn update_one(rappel : Rappel) -> Result<u64, Error> {
    let mut client = database_service::connect()?;
    info!("Updating rappel: {}", rappel.id.expect("Should have and id"));
    let row_update = client.execute("UPDATE rappels SET nom=$1, date_limite=$2, repetition=$3, criticite=$4 WHERE rappel_id=$5", &[&rappel.nom, &rappel.date_limite, &rappel.repetition, &rappel.criticite, &rappel.id])?;
    client.close()?;

    Ok(row_update)
}


pub fn delete_one(id : i32) -> Result<u64, Error> {
    let mut client = database_service::connect()?;
    info!("Deleting rappel: {}", id);
    let statement = client.prepare("DELETE FROM rappels WHERE rappel_id=$1")?;
    let rows = client.execute(&statement, &[&id])?;
    client.close()?;

    Ok(rows)
}