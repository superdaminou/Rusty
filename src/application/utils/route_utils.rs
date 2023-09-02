use dotenv::Error;
use log::info;

use crate::application::utils::route;


pub fn execute_request(parsed_request : Vec<&str>) -> (u16, Option<String>) {
    let ressource: &str  = parsed_request[0];

    info!("Routing call: {}", ressource);

    let http_head = match split(&ressource) {
        Ok(http_head) => http_head,
        Err(error) => return (400, None)
    };
    
    let maybe_route = route::ROUTES.iter().find(|line| exist(http_head, **line));


    let function_to_execute = match maybe_route {
        Some(existing_route) => (existing_route, extract_params(http_head.1, existing_route.1)),
        None => return (404, None) 
    };

    info!("Executing {:?} with following params : {:?}", function_to_execute.0, function_to_execute.1);
    return route::execute(*function_to_execute.0, function_to_execute.1, get_body(&parsed_request));
}

fn split(ressource_line: &str) -> Result<(&str, &str), Error> {
    let splitted_ressource =  ressource_line.split(" ").collect::<Vec<&str>>();
    
    let http_code = splitted_ressource.get(0).unwrap();

    let ressource = *splitted_ressource.get(1).unwrap();

    return Ok((http_code, ressource));
}

fn extract_params(incoming : &str, reference : &str) -> Vec<String>{
    info!("Extracting params from : {}", incoming);
    let mut params : Vec<String>= Vec::new();

    let splitted_entering = incoming.split("/").collect::<Vec<_>>();
    let splitted_reference = reference.split("/").collect::<Vec<_>>();

    for iterator in 0..splitted_entering.len() {
        let reference_part = splitted_reference.get(iterator).unwrap();
        let entering_part = splitted_entering.get(iterator).unwrap();

        if !reference_part.starts_with("{") {
            params.push(entering_part.to_string())
        } 
    }
    return params;

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

fn exist(incoming : (&str, &str), reference : (&str, &str)) -> (bool) {
    if incoming.0.eq_ignore_ascii_case(reference.0) {
        return compare(incoming.1, reference.1);
    } else {
        return false
    }
}

fn compare(incoming : &str,  reference: &str) -> bool {
    let splitted_entering = incoming.split("/").collect::<Vec<_>>();
    let splitted_reference = reference.split("/").collect::<Vec<_>>();

    info!("Start comparing {:?} and {:?}", splitted_entering, splitted_reference);
    if !splitted_entering.len().eq(&splitted_reference.len()) {
        return false;
    }

    for iterator in 0..splitted_entering.len() {
        let reference_part = splitted_reference.get(iterator).unwrap();
        let entering_part = splitted_entering.get(iterator).unwrap();
        if !reference_part.starts_with("{") && !entering_part.eq(reference_part) {
            return false;
        } 
    }
    return true;

}