use crate::application::database::rappel_db_service;
use crate::application::errors::TechnicalError;
use crate::application::rappels::Rappel;
use crate::application::http::structs::response::Response;

pub fn get_rappels() -> Result<Response, TechnicalError> {

    return rappel_db_service::get_all()
        .map_err(|err| TechnicalError::from(err))
        .and_then(|val| serde_json::to_string(&val).map_err(|err| TechnicalError::from(err)))
        .and_then(|body| Ok(Response((200, Some(body)))));
}

pub fn get_rappel(id : i32) -> Result<Response, TechnicalError> {   

    return rappel_db_service::get_one(id)
        .and_then(|val| 
            serde_json::to_string(&val)
            .map_err(|e|TechnicalError::from(e)))
        .and_then(|body| Ok(Response((200, Some(body)))))
        .map_err(|e| TechnicalError::from(e));
}

pub fn add_rappel(rappel : Rappel) -> Result<Response, TechnicalError> {
    return rappel_db_service::add_one(rappel)
        .map_err(|err| TechnicalError::from(err))
        .and_then(|val| Ok(Response((200, Some(val.to_string())))));
}