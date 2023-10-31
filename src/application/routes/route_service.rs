use log::info;
use crate::application::http::structs::response::Response;
use crate::application::routes::route;
use crate::application::http::structs::http_request::HTTPRequest;
use crate::application::routes::route::Route;

pub fn execute_request(request : &str) -> Response {
    let http_request = HTTPRequest::create_from(request).expect("Could not create identifiy request");
    info!("Start executing request: {}", http_request.route);
    
    let maybe_route = route::ROUTES.iter().find(|line| exist(&http_request, line));

    let route = match maybe_route {
        Some(existing_route) => existing_route,
        None => return Response((404,  None))
    };

    return route::execute(route, http_request) 
        .unwrap_or_else(|err| Response((500,Some(err.to_string()))));
}

fn exist(http_request: &HTTPRequest, reference : &Route) -> bool {
    http_request.verb == reference.0 && path_evaluation(&http_request.route, &reference.1)
}

fn path_evaluation(incoming : &str,  reference: &str) -> bool {
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
    return true
}