use std::{str::FromStr, usize};
use log::info;

use crate::http::errors::malformed::MalformedError;

use super::HttpVerb;

pub struct HTTPRequest {
    pub verb: HttpVerb,
    pub route: String,
    pub body: Option<String>
}

#[derive(PartialEq, Debug)]
struct StartLine(String, String);


impl HTTPRequest {

    pub fn request_from(request: &str) -> Result<HTTPRequest, MalformedError> {
        let parsed_request : Vec<String>  = request.trim_matches(char::from(0)).split("\r\n").map(|line| line.to_string()).collect();
        let route = self::get_route(parsed_request.first()).expect("Could not extract route");
        info!("Route to: {} {}", route.0, route.1);

        let body = extract_body(parsed_request)?;

        let verb = HttpVerb::from_str(route.0.as_str()).map_err(|_| MalformedError::from("Parsing error"))?;
        
        return Ok (HTTPRequest { verb: verb, route: route.1, body: Some(body)});
    }


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

fn get_route(ressource_line: Option<&String>) -> Result<StartLine, MalformedError> {
    return ressource_line.ok_or(MalformedError::from("Empty request"))
            .map(|ressource_line|
                ressource_line.split(' ')
                .take(2).collect::<Vec<&str>>())
            .and_then(|decomposed| 
                match (decomposed.get(0), decomposed.get(1)) {
                    (Some(a), Some(b)) =>  Ok(StartLine(a.to_string(), b.to_string())),
                    (_, _) => Err(MalformedError::from("Missing ressource"))
                });

}

fn extract_content_length(request: Vec<String>) -> Result<usize, MalformedError> {
    return request.iter()
            .filter(|line| line.starts_with("Content-Length: "))
            .next()
            .map(|line| line.to_string().drain("Content-Length: ".len()..)
                .as_str()
                .parse::<usize>()
                .map_err(MalformedError::from))
            .unwrap_or(Ok(0));
}

fn extract_body(request: Vec<String> ) -> Result<String, MalformedError> {
    info!("Start Extracting body from request");
    // FIXME: remove clone ? 
    let size = extract_content_length(request.clone())?;
    info!("Content-length : {}", size);


    return request.iter()
                .skip_while(|line|!(*line.trim()).eq(""))
                .skip(1)
                .fold("".to_string(), |acc, e| acc + e)
                .get(0..size)
                .map(|str| str.to_string())
                .ok_or(MalformedError::from("smaller body than expected"));
}




// UNIT TEST
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn extract_content_length_valid_1() {
        let request =vec!["Content-Length: 1".to_string()];
        let result = extract_content_length(request);
        assert_eq!(result, Ok(1));
    }

    #[test]
    fn extract_content_length_invalid_malformed() {
        let request =vec!["Content-Length: a".to_string()];
        let result = extract_content_length(request);
        assert_eq!(result, Err(MalformedError::from("Expected a valid integer")));
    }

    #[test]
    fn extract_content_length_no_headers_0() {
        let request =vec![];
        let result = extract_content_length(request);
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn extract_content_length_multiple_headers_1() {
        let request =vec!["Content-Type: web".to_string(), "Content-Length: 1".to_string()];
        let result = extract_content_length(request);
        assert_eq!(result, Ok(1));
    }


    #[test]
    fn extract_body_exact_size_web() {
        let request =vec!["Content-Length: 3".to_string(), "   ".to_string(), "web".to_string()];
        let result = extract_body(request);
        assert_eq!(result, Ok("web".to_string()));
    }

    #[test]
    fn extract_body_larger_wev() {
        let request =vec!["Content-Length: 3".to_string(), "   ".to_string(), "webuu".to_string()];
        let result = extract_body(request);
        assert_eq!(result, Ok("web".to_string()));
    }

    #[test]
    fn extract_body_shorter_web() {
        let request =vec!["Content-Length: 3".to_string(), "   ".to_string(), "we".to_string()];
        let result = extract_body(request);
        assert_eq!(result, Err(MalformedError::from("smaller body than expected")));
    }


    #[test]
    fn get_route_wrong_malformed_error() {
        let start_line = "No".to_string();
        let result = get_route(Some(&start_line));
        assert_eq!(result, Err(MalformedError::from("Missing ressource")));
    }

    #[test]
    fn get_route_right_get_ok() {
        let start_line = "PUT /url HTTP/1.1".to_string();
        let result = get_route(Some(&start_line));
        assert_eq!(result, Ok(StartLine("PUT".to_string(), "/url".to_string())));
    }
}