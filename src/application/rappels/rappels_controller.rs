use crate::application::database::rappel_db_service;
use crate::application::rappels::Rappel;


pub fn get_rappels() -> (u16, Option<String>) {

    let rappels = rappel_db_service::get_all().unwrap();
    let rappels_json = serde_json::to_string(&rappels).unwrap();

    return (200, Some(rappels_json));
}

pub fn get_rappel(id : i32) -> (u16, Option<String>) {
    
    match rappel_db_service::get_one(id) {
        Ok(results) => {
            match serde_json::to_string(&results) {
                Ok(result) => (200, Some(result)),
                Err(error) => (500, Some(error.to_string()))
            }
        },
        Err(error) => (500, Some(error.to_string())),
    }
}

pub fn add_rappel(rappel : Rappel) -> (u16, Option<String>) {
    match rappel_db_service::add_one(rappel) {
        Ok(result) => (200, Some(result.to_string())),
        Err(error)=> (500, Some(error.to_string()))
    }
}