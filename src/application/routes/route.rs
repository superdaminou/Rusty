use crate::application::errors::TechnicalError;
use crate::application::http::structs::response::Response;
use crate::application::rappels::rappels_controller;
use crate::application::http::structs::http_request::HttpVerb;

#[derive(PartialEq, Eq)]
pub struct Route{
    pub verb  : HttpVerb,pub route:  String,pub  method : fn(ParamsHandler) -> Result<Response, TechnicalError>
}

pub struct ParamsHandler {
    pub params: Vec<String>,
    pub body: Option<String>
}


pub fn routes() -> [Route; 4] {
    let routes = [
        Route {verb: HttpVerb::GET,   route: "/rappels".to_string(), method: rappels_controller::get_rappels},
        Route {verb: HttpVerb::GET, route: "/rappel/{id}".to_string(),method: rappels_controller::get_rappel},
        Route {verb: HttpVerb::POST,   route: "/rappel".to_string(), method: rappels_controller::add_rappel},
        Route {verb: HttpVerb::PUT,   route: "/rappel/{id}".to_string(), method: rappels_controller::update_rappel},
        ];

    return routes;
}
