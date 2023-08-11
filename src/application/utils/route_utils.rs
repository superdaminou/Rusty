use log::info;

use crate::application::rappels::rappels_controller;
use crate::application::utils::route;


pub fn execute_request(request_lines : Vec<&str>) -> (u16, Option<String>) {
    let route  = request_lines[0];

    info!("Routing call: {}", route);

    let associated_methode = match route::ROUTES.iter().find(|line| line.to_string().eq(route)){
        Some(result) => result,
        None => return (404, None)
    };

    match *associated_methode {   
        route::GET_ALL => rappels_controller::get_rappels(),
        route::POST_RAPPEL => rappels_controller::add_rappel(get_body(&request_lines)),
        _ => (404, None)
    }
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