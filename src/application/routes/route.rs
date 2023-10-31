use std::io::Empty;

use crate::application::errors::TechnicalError;
use crate::application::http::structs::response::Response;
use crate::application::rappels::rappels_controller;
use crate::application::http::structs::http_request::HttpVerb;
use crate::application::http::structs::http_request::HTTPRequest;
use crate::application::rappels::structures::Rappel;
use crate::application::http::structs::http_response::HTTPResponse;
use log::info;

#[derive(PartialEq, Eq)]
pub struct Route<'a>(pub HttpVerb,pub &'a str);

pub const GET_RAPPELS : Route = Route(HttpVerb::GET, "/rappels");
pub const POST_RAPPEL : Route = Route(HttpVerb::POST, "/rappel");
pub const GET_RAPPEL : Route = Route(HttpVerb::GET, "/rappel/{id}");
pub const PUT_RAPPEL : Route = Route(HttpVerb::PUT, "/rappel/{id}");
pub const NOT_FOUND : Route = Route(HttpVerb::GET, "/not_found");

pub const ROUTES : [Route; 4] = [
    GET_RAPPELS, POST_RAPPEL, GET_RAPPEL, PUT_RAPPEL
];


pub fn execute(route : &Route, request : HTTPRequest) -> Result<Response, TechnicalError> {
    info!("Executing {:?}", request.route);
    let params = request.extract_params(route.1);
    let response = match route {   
        &GET_RAPPELS => rappels_controller::get_rappels(),
        &POST_RAPPEL => rappels_controller::add_rappel(as_rappel(request.body)),
        &GET_RAPPEL => rappels_controller::get_rappel(as_int(params.get(0))),
        &PUT_RAPPEL => rappels_controller::get_rappel(as_int(params.get(0))),
        &NOT_FOUND => Ok(Response((404, None))),
        _ => Ok(Response((404, None)))
    };
    return response;
}

fn as_int(var : Option<&String>) -> i32 {
    var.unwrap().parse::<i32>().unwrap().to_owned()
}

fn as_rappel(body: Option<String>) -> Rappel {
    return serde_json::from_str(&body.unwrap()).unwrap();
}