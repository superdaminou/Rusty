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
        .map_err(|e| TechnicalError::from(e))
        .and_then(|rappel| {
            return match rappel {
                None => Ok(Response((404, None))),
                Some(rappel) => {
                    return serde_json::to_string(&rappel)
                    .map_err(|e|TechnicalError::from(e))
                    .and_then(|result| Ok(Response((200, Some(result)))));
                }
            }
    });
}

pub fn add_rappel(rappel : Rappel) -> Result<Response, TechnicalError> {
    return rappel_db_service::add_one(rappel)
        .map_err(|err| TechnicalError::from(err))
        .and_then(|val| Ok(Response((200, Some(val.to_string())))));
}

pub fn update_rappel(rappel : Rappel) -> Result<Response, TechnicalError> {
    return match rappel_db_service::get_one(rappel.id).is_ok() {
        true => {
            return rappel_db_service::update_one(rappel)
                .map_err(|err| TechnicalError::from(err))
                .and_then(|val| Ok(Response((200, Some(val.to_string())))))
        },
        false => Ok(Response((404, None)))
    };
}
