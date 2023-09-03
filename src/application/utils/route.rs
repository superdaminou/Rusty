use crate::application::rappels::rappels_controller;
use crate::application::utils::http_request::HttpVerb;
use crate::application::utils::http_request::HTTPRequest;


pub const GET_ALL : (HttpVerb, &str) = (HttpVerb::GET,"/all");
pub const POST_RAPPEL : (HttpVerb, &str)  = (HttpVerb::POST, "/rappel");
pub const GET_ONE_RAPPEL : (HttpVerb, &str) = (HttpVerb::GET, "/{id}");

pub const ROUTES : [(HttpVerb, &str); 3] = [GET_ALL, 
                                POST_RAPPEL,
                                GET_ONE_RAPPEL];


pub fn execute(route : &(HttpVerb, &str), request : HTTPRequest) -> (u16, Option<String>) {
    let params = request.extract_params(route.1);
    return match route {   
        &GET_ALL => rappels_controller::get_rappels(),
        &POST_RAPPEL => rappels_controller::add_rappel(request.body),
        &GET_ONE_RAPPEL => rappels_controller::get_rappel(as_int(params.get(0))),
        _ => (404, None)
    }
}

fn as_int(var : Option<&String>) -> i32 {
    var.unwrap().parse::<i32>().unwrap().to_owned()
}