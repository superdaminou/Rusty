use crate::application::database::rappel_db_service;
use crate::application::errors::TechnicalError;
use rustyttp::Response;
use rustyttp::ParamsHandler;
use crate::application::rappels::structs::adatapers::json_update_rappel::UpdateRappel;
use log::info;

use super::structs::adatapers::json_write_rappel::WriteRappel;
use super::structs::Rappel;

pub fn get_rappels(_handler: ParamsHandler) -> Response {

    return rappel_db_service::get_all()
        .map_err(|err| TechnicalError::from(err))
        .and_then(|val| serde_json::to_string(&val).map_err(|err| TechnicalError::from(err)))
        .map(|body| Response::from((200, body)))
        .unwrap_or_else(|err| Response::from(err));
}

pub fn get_rappel(handler: ParamsHandler) -> Response {   
    return handler.params
            .get("id")
            .map(|id| 
                id.parse::<i32>()
                .map_err(|err| TechnicalError::from("value")))
            .unwrap_or_else(|| Err(TechnicalError::from("Missing Id")))
            .map(|id| rappel_db_service::get_one(id))
            .unwrap_or_else(|err| Err(err))
            .map(|rappel|  
                serde_json::to_string(&rappel)
                .map_err(|_|TechnicalError::from("serialisation error")))
            .map(|rappel| rappel.map_or(Response::from((404, "Not found")), |rappel|  Response::from((200, rappel))))
            .unwrap_or_else(|err| Response::from(err));
}


pub fn add_rappel(handler : ParamsHandler) -> Response {
    info!("Start adding rappel");

    return handler.body.iter()
        .next()
        .map(|body| WriteRappel::extract(body.to_string()))
        .unwrap_or(Err(TechnicalError::from("missing body")))
        .map(|rappel| 
            rappel_db_service::add_one(Rappel::from(rappel))
            .map_err(|err|TechnicalError::from(err)))
        .map(|result| 
            match result {
                Err(error) => Response::from(error),
                Ok(body)=> Response::from((200, body.to_string()))
            })
        .unwrap_or_else(|err| Response::from(err));
}

pub fn update_rappel(handler : ParamsHandler) -> Response {
    info!("Start updating");

    return handler.body.iter()
        .next()
        .ok_or(TechnicalError::from("Missing body"))
        .map(|body| UpdateRappel::extract(body.to_string()))
        .unwrap_or_else(|err| Err(err))
        .map(|rappel| 
            rappel_db_service::update_one(Rappel::from(rappel))
            .map_err(|err| TechnicalError::from(err)))
        .map(|result|
            match result {
                Ok(body) => Response::from((200, body.to_string())),
                Err(err) => Response::from(err)
            } )
        .unwrap_or_else(|err| Response::from(err));

}


pub fn delete_rappel(handler : ParamsHandler) -> Response {
    info!("Start Deleting");

    return handler.params
        .get("id")
        .map(|id| 
            id.parse::<i32>()
            .map_err(|err| TechnicalError::from("Parse error")))
        .unwrap_or_else(|| Err(TechnicalError::from("Missing Id")))
        .map(|id| rappel_db_service::get_one(id))
        .unwrap_or_else(|err| Err(err))
        .map(|rappel| 
            rappel_db_service::delete_one(rappel.id.unwrap())
            .map_err(|err| TechnicalError::from(err)))
        .map(|rows| 
            match rows {
                Err(err) => Response::from(err),
                Ok(rows) => Response::from((200, rows.to_string()))
            } )
        .unwrap_or_else(|err| Response::from(err));
}