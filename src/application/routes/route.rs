use crate::application::rappels::rappels_controller;
use crate::application::http::structs::http_request::HttpVerb;
use crate::application::http::structs::http_request::HTTPRequest;
use crate::application::rappels::structures::Rappel;
use crate::application::http::structs::http_response::HTTPResponse;
use dotenv::Error;
use log::info;

#[derive(PartialEq, Eq)]
pub struct Route<'a>(pub HttpVerb,pub &'a str);

pub const GET_RAPPELS : Route = Route(HttpVerb::GET, "/rappels");
pub const POST_RAPPEL : Route = Route(HttpVerb::POST, "/rappel");
pub const GET_RAPPEL : Route = Route(HttpVerb::GET, "/rappel/{id}");
pub const PUT_RAPPEL : Route = Route(HttpVerb::PUT, "/rappel/{id}");

pub const ROUTES : [Route; 4] = [
    GET_RAPPELS, POST_RAPPEL, GET_RAPPEL, PUT_RAPPEL
];


pub fn execute(route : &Route, request : HTTPRequest) -> Result<HTTPResponse, Error> {
    info!("Executing {:?}", request.route);
    let params = request.extract_params(route.1);
    return match route {   
        &GET_RAPPELS => Ok(rappels_controller::get_rappels()),
        &POST_RAPPEL => Ok(rappels_controller::add_rappel(as_rappel(request.body))),
        &GET_RAPPEL => Ok(rappels_controller::get_rappel(as_int(params.get(0)))),
        &PUT_RAPPEL => Ok(rappels_controller::get_rappel(as_int(params.get(0)))),
        _ => Ok(HTTPResponse {code: 404, body: None})
    }
}

fn as_int(var : Option<&String>) -> i32 {
    var.unwrap().parse::<i32>().unwrap().to_owned()
}

fn as_rappel(body: Option<String>) -> Rappel {
    return serde_json::from_str(&body.unwrap()).unwrap();
}