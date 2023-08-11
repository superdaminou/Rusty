#[macro_use]
extern crate log;
extern crate dotenv;

mod rappels;
mod database_service;
mod route;

use notification_playground::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use log::{warn, log_enabled, debug};
use dotenv::dotenv;
use std::env;

use rappels::Rappel;

// Listen on port 7878 and for each stream incoming create a new thread.
fn main() {
    dotenv().ok();
    env_logger::init();
    


    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    warn!("Shutting down.");
}

// Router
fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();


    let request: &str = str::from_utf8(&buffer).unwrap();
    
    warn!("Entering request {:?}", request);

    let request_lines : Vec<&str> = request.trim_matches(char::from(0)).split("\r\n").collect();

    let (status_line , contents ) = match route::ROUTES.contains(&request_lines[0]) {
        true => get_call(request_lines),
        false => ("HTTP/1.1 404 NOT FOUND", "Not found".to_string())
    };

    let response = format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        "application/json",
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_call(request_lines : Vec<&str>) -> (&str, String) {
    let route = request_lines[0];
    //let method_called = get_associed_method(route.to_string());

    warn!("Routing call: {}", route);

    return match route {
        route::GET_ALL => {
            let result = rappels::get_rappels();
            ("HTTP/1.1 200 OK", result)
        },
        route::POST_RAPPEL => {

            let body: String = match get_body_if_exist(&request_lines) {
                Some(result) => result,
                None => return ("HTTP/1.1 403 Forbiden", "Body mandatory".to_string())
            };

            warn!("Trying to deserialize : {}", &body);
            let rappel: Rappel = match serde_json::from_str(&body[..body.len()-1]) {
                Ok(rappel) => rappel,
                Err(err) => return ("HTTP/1.1 500 KO", format!("Errors while parsing json {} stacktrace {}",&body, err.to_string()))
            };


            return ("HTTP/1.1 200 OK", rappels::new_rappel(rappel).to_string());
        },
        _ => ("HTTP/1.1 400 Not found", "".to_string())
    };

}

//fn get_associed_method<T> (route : String) -> impl Fn(T) -> Option<T> {
//    return rappels::get_all();
// } 

fn get_body_if_exist(request: &Vec<&str> ) -> Option<String> {
    warn!("Start Extracting body from request");

    let content_length_position = match request.iter().position(|line| line.starts_with("Content-Length: ")) {
        Some(position) => position,
        None => return None
    };

    let content_length_line: &&str = &request[content_length_position];
    
    let size = match content_length_line["Content-Length: ".len()..].parse::<usize>() {
        Ok(size) => size,
        Err(e) => panic!("{}", e)
    };

    warn!("Content-length : {}", size);

    let body: String  = request.clone()
        .drain(content_length_position+1..)
        .as_slice()
        .concat();

    warn!("Extracted body : {}", body);
    return Some(body);
}
