use dotenv::Error;
use strum_macros::Display;
use std::str::FromStr;
use log::info;
extern crate strum;

pub struct HTTPRequest {
    pub verb: HttpVerb,
    pub route: String,
    pub body: Option<String>
}

impl HTTPRequest {

    pub fn create_from(request: &str) -> HTTPRequest {
        let parsed_request : Vec<&str> = request.trim_matches(char::from(0)).split("\r\n").collect();
        let route = self::get_route(parsed_request[0]).unwrap();
        let body = get_body(&parsed_request);

        
        HTTPRequest { verb: HttpVerb::from_str(route.0).unwrap(), route: route.1.to_string(), body }
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

fn get_route(ressource_line: &str) -> Result<(&str, &str), Error> {
    let splitted_ressource =  ressource_line.split(' ').collect::<Vec<&str>>();
    
    let http_code = splitted_ressource.first().unwrap();

    let ressource = *splitted_ressource.get(1).unwrap();

    Ok((http_code, ressource))
}


fn get_body(request: &[&str] ) -> Option<String> {
    info!("Start Extracting body from request");

    let content_length_position = match request.iter().position(|line| line.starts_with("Content-Length: ")) {
        Some(position) => position,
        None => return None
    };

    let content_length_line: &&str = &request[content_length_position];
    
    let size = match content_length_line["Content-Length: ".len()..].parse::<usize>() {
        Ok(size) => size,
        Err(e) => panic!("{}", e)
    };

    info!("Content-length : {}", size);

    let body: String  = request.to_owned()
        .drain(content_length_position+1..)
        .as_slice()
        .concat();

    info!("Extracted body : {}", body);
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