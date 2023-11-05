use crate::application::errors::TechnicalError;
use crate::application::rappels::structures::Rappel;
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


pub fn get_one(id: i32) -> Result<Option<Rappel>, TechnicalError> {
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
        0  => Ok(None),
        1 => Ok(Some(rappels.iter().clone().next().unwrap().clone())),
        _ => Err(TechnicalError::from("Should have 1 result max".to_string()))
    };
}


pub fn update_one(rappel : Rappel) -> Result<u64, Error> {
    let mut client = database_service::connect()?;
    info!("Updating rappel: {}", rappel.id);
    let row_update = client.execute("UPDATE rappels SET nom=$1, date_limite=$2, repetition=$3, criticite=$4 WHERE rappel_id=$5", &[&rappel.nom, &rappel.date_limite, &rappel.repetition, &rappel.criticite, &rappel.id])?;
    client.close()?;

    Ok(row_update)
}