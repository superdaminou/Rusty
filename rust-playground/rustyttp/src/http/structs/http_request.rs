use std::{str::FromStr, usize};
use log::info;

use crate::http::errors::malformed::MalformedError;

use super::HttpVerb;

pub struct HTTPRequest {
    pub verb: HttpVerb,
    pub route: String,
    pub body: Option<String>
}


impl From<&str> for HTTPRequest {
    fn from(request : &str) -> Self {
        create_from(request).unwrap_or(HTTPRequest { verb: HttpVerb::GET, route: "/malformed".to_string(), body: None })
    }
}


fn create_from(request: &str) -> Result<HTTPRequest, MalformedError> {
    let parsed_request : Vec<String>  = request.trim_matches(char::from(0)).split("\r\n").map(|line| line.to_string()).collect();
    let route = self::get_route(parsed_request.first()).expect("Could not extract route");
    info!("Route to: {} {}", route.0, route.1);

    let body = get_body(parsed_request);

    let verb = HttpVerb::from_str(route.0.as_str()).map_err(|_| MalformedError::from("Parsing error"))?;
    
    return Ok (HTTPRequest { verb: verb, route: route.1, body });
}


impl HTTPRequest {

    pub fn extract_params(&self, reference : String) -> Vec<String>{
        info!("Extracting params from : {}", self.route);
        let mut params : Vec<String>= Vec::new();
    
        let splitted_entering = self.route.split('/').collect::<Vec<_>>();
        let splitted_reference = reference.split('/').collect::<Vec<_>>();
    
        for iterator in 0..splitted_entering.len() {
            let reference_part = splitted_reference.get(iterator).unwrap();
            let entering_part = splitted_entering.get(iterator).unwrap();
    
            if reference_part.starts_with('{') {
                params.push(entering_part.to_string())
            } 
        }
        params
    
    }


}

fn get_route(ressource_line: Option<&String>) -> Result<(String, String), MalformedError> {
    let splitted_ressource : Vec<String> =  ressource_line
            .expect("Cannot extract route")
            .split(' ')
            .take(2)
            .map(|line| line.to_string())
            .collect();

    
    let http_code = splitted_ressource.get(0)
        .expect("Can't find HTTP CODE")
        .to_string();
    let ressource = splitted_ressource.get(1)
        .expect("Cant find ressource")
        .to_string();

    Ok((http_code, ressource))
}


fn get_body(request: Vec<String> ) -> Option<String> {
    info!("Start Extracting body from request");

    let content_length_position =match request.iter().position(|line| line.starts_with("Content-Length: ")) {
        None => return None,
        Some(position) => position
    };


    let size = request
        .iter()
        .nth(content_length_position)
        .expect("Expected a Content-Length")
        .to_owned()
        .drain("Content-Length: ".len()..)
        .collect::<String>()
        .parse::<usize>()
        .expect("Should be a valid number");
    
    info!("Content-length : {}", size);
    
    
    let body = request
        .iter()
        .skip(request.iter().position(|value| (*value).starts_with("{"))?)
        .flat_map(|s| s.chars())
        .collect();

    info!("Body: {}", body);
    Some(body)
}