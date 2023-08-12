use notification_playground::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use log::info;


use crate::application::utils::route_utils;

const PROTOCOL : &str= "HTTP/1.1";

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


// PRIVATE

// Router
fn handle_connection(mut stream: TcpStream) {
    info!("Handling Connection");
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let request: &str = str::from_utf8(&buffer).unwrap();
    let request_lines : Vec<&str> = request.trim_matches(char::from(0)).split("\r\n").collect();

    info!("Routing: {}", &request_lines[0]);
    
    let (code, contents ) = route_utils::execute_request(request_lines);

    let response = constructing_response(code, contents);

    write(stream, response);
}

fn write(mut stream : TcpStream, response: String) {
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn constructing_response(code : u16, mut contents : Option<String>) -> String {
    let status_line: String= construct_status_line(code);

    let content = contents.get_or_insert("".to_string());

    format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        "application/json",
        content.len(),
        content
    )
}

fn construct_status_line(code : u16) -> String {
    format!("{} {} {}", PROTOCOL, code, message_from_code(code))
}

fn message_from_code(code : u16) -> String {
    match code {
        200 =>  "OK".to_string(),
        500 => "INTERNAL".to_string(),
        404 => "NOT FOUND".to_string(),
        _  => "WTF".to_string()
    }
}