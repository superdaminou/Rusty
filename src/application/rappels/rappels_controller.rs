use crate::application::database::rappel_db_service;
use crate::application::errors::TechnicalError;
use crate::application::rappels::Rappel;
use crate::application::http::structs::http_response::HTTPResponse;

pub fn get_rappels() -> Result<HTTPResponse, TechnicalError> {

    return rappel_db_service::get_all()
        .map_err(|err| TechnicalError::new(err.to_string()))
        .and_then(|val| serde_json::to_string(&val).map_err(|err| TechnicalError::new(err.to_string())))
        .and_then(|body| Ok(toHttpResponse(body)));
}

pub fn get_rappel(id : i32) -> Result<HTTPResponse, TechnicalError> {   

    return rappel_db_service::get_one(id)
        .map_err(|e| TechnicalError::new(e.to_string()))
        .and_then(|val| serde_json::to_string(&val).map_err(|e|TechnicalError::new(e.to_string())))
        .and_then(|body| Ok(toHttpResponse(body)));
}

pub fn add_rappel(rappel : Rappel) -> Result<HTTPResponse, TechnicalError> {
    return rappel_db_service::add_one(rappel)
        .map_err(|err| TechnicalError::new(err.to_string()))
        .and_then(|val| Ok(toHttpResponse(val.to_string())));
}

fn toHttpResponse(body: String) -> HTTPResponse {
    return HTTPResponse {code: 200, body: Some(body)};
}

fn to500(error: String) -> HTTPResponse {
    return HTTPResponse {code: 500, body: Some(error)};
}