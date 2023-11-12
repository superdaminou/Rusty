use log::info;
use route::ParamsHandler;
use crate::application::http::structs::response::Response;
use crate::application::routes::route;
use crate::application::http::structs::http_request::HTTPRequest;
use crate::application::routes::route::Route;


pub fn execute_request(http_request : HTTPRequest) -> Response {
    info!("Start executing request: {}", http_request.route);
    
     return route::routes().iter()
        .find(|route| equals(&http_request, &route))
        .map_or(
            Ok(Response((404, None))) 
            ,|route| (route.method)(ParamsHandler { params : http_request.extract_params(route.route.clone()), body: http_request.body } ))
        .unwrap_or_else(|err| Response((500, Some(err.to_string()))));
}

fn equals(http_request: &HTTPRequest, reference : &Route) -> bool {
    http_request.verb == reference.verb && path_evaluation(&http_request.route, &reference.route)
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


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn path_equals() {
        let result = path_evaluation("/path", "/path");
        assert_eq!(result, true);
    }


    #[test]
    fn path_not_equals() {
        let result = path_evaluation("/paths", "/path");
        assert_eq!(result, true);
    }
}