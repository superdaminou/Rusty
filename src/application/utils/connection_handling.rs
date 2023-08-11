use notification_playground::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use log::info;

use crate::application;
use crate::application::utils::route;

pub fn open_connection(){
    info!("Opening connection and listening");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

// Router
fn handle_connection(mut stream: TcpStream) {
    info!("Handling Connection");
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();


    let request: &str = str::from_utf8(&buffer).unwrap();
    let request_lines : Vec<&str> = request.trim_matches(char::from(0)).split("\r\n").collect();

    info!("Routing: {}", &request_lines[0]);
    
    let (status_line , contents ) = match route::ROUTES.contains(&request_lines[0]) {
        true => get_call(request_lines),
        false => ("HTTP/1.1 404 NOT FOUND", "Not found".to_string())
    };

    let response = constructing_response(status_line, contents);

    write(stream, response);
}

fn write(mut stream : TcpStream, response: String) {
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn constructing_response(status_line : &str, contents : String) -> String {
    return format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        "application/json",
        contents.len(),
        contents
    );
}

fn get_call(request_lines : Vec<&str>) -> (&str, String) {
    let route = request_lines[0];

    info!("Routing call: {}", route);

    return match route {
        route::GET_ALL => {
            let result = application::rappels::get_rappels();
            ("HTTP/1.1 200 OK", result)
        },
        route::POST_RAPPEL => application::rappels::add_rappel(get_body(&request_lines)),
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