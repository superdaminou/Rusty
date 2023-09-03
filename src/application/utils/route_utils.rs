use log::info;
use crate::application::utils::http_request::HttpVerb;
use crate::application::utils::route;

use super::http_request::HTTPRequest;

pub fn execute_request(request : &str) -> (u16, Option<String>) {
    let http_request = HTTPRequest::create_from(request);
    
    let maybe_route = route::ROUTES.iter().find(|line| exist(&http_request, line));

    let route = match maybe_route {
        Some(existing_route) => existing_route,
        None => return (404, None) 
    };

    route::execute(route, http_request)
}


fn exist(http_request: &HTTPRequest, reference : &(HttpVerb, &str)) -> bool {
    info!("Does {} {} exist", http_request.verb, http_request.route);
    http_request.verb == reference.0 && compare(&http_request.route, reference.1)
}

fn compare(incoming : &str,  reference: &str) -> bool {
    let splitted_entering = incoming.split('/').collect::<Vec<_>>();
    let splitted_reference = reference.split('/').collect::<Vec<_>>();

    info!("Start comparing {:?} and {:?}", splitted_entering, splitted_reference);
    if !splitted_entering.len().eq(&splitted_reference.len()) {
        return false;
    }

    for iterator in 0..splitted_entering.len() {
        let reference_part = splitted_reference.get(iterator).unwrap();
        let entering_part = splitted_entering.get(iterator).unwrap();
        if !reference_part.starts_with('{') && !entering_part.eq(reference_part) {
            return false;
        } 
    }
    true

}