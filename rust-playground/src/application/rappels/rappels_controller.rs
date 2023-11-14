use crate::application::database::rappel_db_service;
use crate::application::errors::TechnicalError;
use crate::application::http::structs::response::Response;
use crate::application::rappels::structs::adatapers::json_update_rappel::UpdateRappel;
use crate::application::routes::ParamsHandler;
use log::info;

use super::structs::adatapers::json_write_rappel::WriteRappel;
use super::structs::Rappel;

pub fn get_rappels(_handler: ParamsHandler) -> Result<Response, TechnicalError> {

    return rappel_db_service::get_all()
        .map_err(|err| TechnicalError::from(err))
        .and_then(|val| serde_json::to_string(&val).map_err(|err| TechnicalError::from(err)))
        .and_then(|body| Ok(Response((200, Some(body)))));
}

pub fn get_rappel(handler: ParamsHandler) -> Result<Response, TechnicalError> {   
    return handler.params
            .iter()
            .next()
            .ok_or(TechnicalError::from("Parsing Error"))
            .map(|id| id.parse::<i32>())?
            .map_err(|_| TechnicalError::from("Parsing Error"))
            .map(|id| rappel_db_service::get_one(id))?
            .map(|rappel|  
                serde_json::to_string(&rappel)
                .map_err(|_|TechnicalError::from("serialisation error")))
            .map(|rappel| rappel.map_or(Response((404, Some("Not found".to_string()))), |rappel|  Response((200, Some(rappel)))));
}


pub fn add_rappel(handler : ParamsHandler) -> Result<Response, TechnicalError> {
    let rappel = match handler.body.iter().next() {
        None => return Err(TechnicalError::from("value")),
        Some(body) => WriteRappel::from(body.clone()) 
    };

    return rappel_db_service::add_one(Rappel::from(rappel))
        .map_err(|err| TechnicalError::from(err))
        .and_then(|val| Ok(Response((200, Some(val.to_string())))));
}

pub fn update_rappel(handler : ParamsHandler) -> Result<Response, TechnicalError> {
    info!("Start updating");
    let rappel = match handler.body.iter().next() {
        None => return Err(TechnicalError::from("nooo")),
        Some(body) => UpdateRappel::from(body.clone()) 
    };

    return match rappel_db_service::get_one(rappel.id).is_ok() {
        true => {
            return rappel_db_service::update_one(Rappel::from(rappel))
                .map_err(|err| TechnicalError::from(err))
                .and_then(|val| Ok(Response((200, Some(val.to_string())))))
        },
        false => Ok(Response((404, None)))
    };
}


pub fn delete_rappel(handler : ParamsHandler) -> Result<Response, TechnicalError> {
    info!("Start Deleting");

    return handler.params
        .iter()
        .next()
        .ok_or(TechnicalError::from("Malformed"))
        .map(|id| id.parse::<i32>())?
        .map_err(|err| TechnicalError::from("Parsed Error"))
        .map(|id| rappel_db_service::get_one(id))?
        .map(|rappel| rappel.ok_or(TechnicalError::from("Not found")))?
        .map(|rappel| rappel_db_service::delete_one(rappel.id.unwrap()).map_err(|err| TechnicalError::from(err)))?
        .and_then(|rows| Ok(Response((200, Some(rows.to_string())))));
}