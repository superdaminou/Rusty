use log::info;
use crate::application::database::rappel_db_service;
use crate::application::rappels::Rappel;


pub fn get_rappels() -> (u16, Option<String>) {
    match rappel_db_service::get_all() {
        Ok(results) => {
            match serde_json::to_string(&results) {
                Ok(result) => (200, Some(result)),
                Err(error) => (500, Some(error.to_string()))
            }
        },
        Err(error) => (500, Some(error.to_string())),
    }
}

pub fn get_rappel(id : u128) -> (u16, Option<String>) {
    match rappel_db_service::get_all() {
        Ok(results) => {
            match serde_json::to_string(&results) {
                Ok(result) => (200, Some(result)),
                Err(error) => (500, Some(error.to_string()))
            }
        },
        Err(error) => (500, Some(error.to_string())),
    }
}


pub fn add_rappel(body : Option<String>) -> (u16, Option<String>) {

    let body: String = match body {
        Some(result) => result,
        None => return (403, Some("Body mandatory".to_string()))
    };
    

    info!("Trying to deserialize : {}", body);
    let rappel: Rappel = match serde_json::from_str(&body) {
        Ok(rappel) => rappel,
        Err(err) => return (500, Some(err.to_string()))
    };

    match rappel_db_service::add_one(rappel) {
        Ok(result) => (200, Some(result.to_string())),
        Err(error)=> (500, Some(error.to_string()))
    }
}