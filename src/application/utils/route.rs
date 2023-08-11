use log::info;

const GET_ALL : &str = "GET /all HTTP/1.1";
const POST_RAPPEL : &str  = "POST /rappel HTTP/1.1";

use crate::application::rappels::rappels;


pub const ROUTES: [&str; 2] = [GET_ALL, POST_RAPPEL];


fn route(request_lines : Vec<&str>) -> (&str, String) {
    let route = request_lines[0];

    info!("Routing call: {}", route);

    return match route {
        GET_ALL => {
            let result = rappels::get_rappels();
            ("HTTP/1.1 200 OK", result)
        },
        POST_RAPPEL => rappels::add_rappel(get_body(&request_lines)),
        _ => ("HTTP/1.1 400 Not found", "".to_string())
    };

}


fn get_body(request: &Vec<&str> ) -> Option<String> {
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

    let body: String  = request.clone()
        .drain(content_length_position+1..)
        .as_slice()
        .concat();

    info!("Extracted body : {}", body);
    return Some(body);
}

pub fn execute_request(request_lines : Vec<&str>) -> (&str, String) {
    return match ROUTES.contains(&request_lines[0]) {
        true => route(request_lines),
        false => ("HTTP/1.1 404 NOT FOUND", "Not found".to_string())
    };
}