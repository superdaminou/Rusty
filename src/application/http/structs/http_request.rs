use dotenv::Error;
use strum_macros::Display;
use std::{str::FromStr, usize};
use log::info;
extern crate strum;

pub struct HTTPRequest {
    pub verb: HttpVerb,
    pub route: String,
    pub body: Option<String>
}

impl HTTPRequest {

    pub fn create_from(request: &str) -> Result<HTTPRequest, Error> {
        let parsed_request : Vec<String>  = request.trim_matches(char::from(0)).split("\r\n").map(|line| line.to_string()).collect();
        let route = self::get_route(parsed_request.first()).expect("Could not extract route");
        let body = get_body(parsed_request);


        
        return Ok (HTTPRequest { verb: HttpVerb::from_str(route.0.as_str()).unwrap(), route: route.1, body });
    }


    pub fn extract_params(&self, reference : &str) -> Vec<String>{
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

fn get_route(ressource_line: Option<&String>) -> Result<(String, String), Error> {
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


    // FIXME Skip if no content lenght 
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
        .skip(content_length_position)
        .flat_map(|s| s.chars())
        .collect();
    Some(body)
}


#[derive(PartialEq, Eq, Display)]
pub enum HttpVerb {
    POST,
    GET,
    PUT,
    DELETE,
    PATCH
}

impl FromStr for HttpVerb {
    type Err = ();

    fn from_str(input: &str) -> Result<HttpVerb, Self::Err> {
        match input {
            "POST" => Ok(Self::POST),
            "GET" => Ok(Self::GET),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(())
        }
    }
}