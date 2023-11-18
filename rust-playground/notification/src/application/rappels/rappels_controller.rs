use crate::application::database::rappel_db_service;
use crate::application::errors::TechnicalError;
use rustyttp::http::structs::response::Response;
use crate::application::rappels::structs::adatapers::json_update_rappel::UpdateRappel;
use crate::application::routes::ParamsHandler;
use log::info;

use super::structs::adatapers::json_write_rappel::WriteRappel;
use super::structs::Rappel;

pub fn get_rappels(_handler: ParamsHandler) -> Result<Response, TechnicalError> {

    return rappel_db_service::get_all()
        .map_err(|err| TechnicalError::from(err))
        .and_then(|val| serde_json::to_string(&val).map_err(|err| TechnicalError::from(err)))
        .and_then(|body| Ok(Response::from((200, body))));
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
            .map(|rappel| rappel.map_or(Response::from((404, "Not found")), |rappel|  Response::from((200, rappel))));
}


pub fn add_rappel(handler : ParamsHandler) -> Result<Response, TechnicalError> {
    info!("Start adding rappel");
    return handler.body.iter()
        .next()
        .ok_or(TechnicalError::from("Missing body"))
        .map(|body| WriteRappel::extract(body.to_string()))?
        .map(|rappel| 
            rappel_db_service::add_one(Rappel::from(rappel))
            .map_err(|err|TechnicalError::from(err)))?
        .and_then(|result| Ok(Response::from((200, result.to_string()))));
}

pub fn update_rappel(handler : ParamsHandler) -> Result<Response, TechnicalError> {
    info!("Start updating");

    return handler.body.iter()
        .next()
        .ok_or(TechnicalError::from("Missing body"))
        .map(|body| UpdateRappel::extract(body.to_string()))?
        .map(|rappel| 
            rappel_db_service::update_one(Rappel::from(rappel))
            .map_err(|err| TechnicalError::from(err)))?
        .and_then(|result| Ok(Response::from((200, result.to_string()))));

}


pub fn delete_rappel(handler : ParamsHandler) -> Result<Response, TechnicalError> {
    info!("Start Deleting");

    return handler.params
        .iter()
        .next()
        .ok_or(TechnicalError::from("Malformed"))
        .map(|id| id.parse::<i32>())?
        .map_err(|_| TechnicalError::from("Parsed Error"))
        .map(|id| rappel_db_service::get_one(id))?
        .map(|rappel| rappel.ok_or(TechnicalError::from("Not found")))?
        .map(|rappel| rappel_db_service::delete_one(rappel.id.unwrap()).map_err(|err| TechnicalError::from(err)))?
        .and_then(|rows| Ok(Response::from((200, rows.to_string()))));
}