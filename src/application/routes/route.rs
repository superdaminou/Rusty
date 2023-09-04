use crate::application::rappels::rappels_controller;
use crate::application::http::structs::http_request::HttpVerb;
use crate::application::http::structs::http_request::HTTPRequest;
use crate::application::rappels::structures::Rappel;
use dotenv::Error;

pub const GET_ALL : (HttpVerb, &str) = (HttpVerb::GET,"/all");
pub const POST_RAPPEL : (HttpVerb, &str)  = (HttpVerb::POST, "/rappel");
pub const GET_ONE_RAPPEL : (HttpVerb, &str) = (HttpVerb::GET, "/{id}");

pub const ROUTES : [(HttpVerb, &str); 3] = [GET_ALL, 
                                POST_RAPPEL,
                                GET_ONE_RAPPEL];


pub fn execute(route : &(HttpVerb, &str), request : HTTPRequest) -> Result<(u16, Option<String>), Error> {
    let params = request.extract_params(route.1);
    return match route {   
        &GET_ALL => Ok(rappels_controller::get_rappels()),
        &POST_RAPPEL => Ok(rappels_controller::add_rappel(as_rappel(request.body))),
        &GET_ONE_RAPPEL => Ok(rappels_controller::get_rappel(as_int(params.get(0)))),
        _ => Ok((404, None))
    }
}

fn as_int(var : Option<&String>) -> i32 {
    var.unwrap().parse::<i32>().unwrap().to_owned()
}

fn as_rappel(body: Option<String>) -> Rappel {
    return serde_json::from_str(&body.unwrap()).unwrap();
}