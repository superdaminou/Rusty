use crate::application::database::rappel_db_service;
use crate::application::rappels::Rappel;
use crate::application::http::structs::http_response::HTTPResponse;

pub fn get_rappels() -> HTTPResponse {

    let rappels = rappel_db_service::get_all().unwrap();
    let rappels_json = serde_json::to_string(&rappels).unwrap();

    return HTTPResponse {code: 200, body: Some(rappels_json)}
}

pub fn get_rappel(id : i32) -> HTTPResponse {    
    match rappel_db_service::get_one(id) {
        Ok(results) => {
            match serde_json::to_string(&results) {
                Ok(result) => HTTPResponse {code: 200, body: Some(result)},
                Err(error) => HTTPResponse{code: 500, body: Some(error.to_string())}
            }
        },
        Err(error) => HTTPResponse {code: 500, body: Some(error.to_string())},
    }
}

pub fn add_rappel(rappel : Rappel) -> HTTPResponse {
    match rappel_db_service::add_one(rappel) {
        Ok(result) => HTTPResponse {code: 200, body: Some(result.to_string())},
        Err(error)=> HTTPResponse {code: 500, body: Some(error.to_string())}
    }
}