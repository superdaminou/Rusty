use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use log::info;
use std::env;
use crate::application::routes::route_service;
use crate::application::http::structs::thread_pool::ThreadPool;
use crate::application::http::structs::http_response::HTTPResponse;

const PROTOCOL : &str= "HTTP/1.1";

pub fn open_connection(){
    info!("Opening connection and listening");

    let adresse = env::var("SERVER_ADRESS").unwrap_or("127.0.0.1:7878".to_string());
    info!("Start listening on {}", adresse);
    let listener = TcpListener::bind(adresse).unwrap();
    info!("Initializing thread pool : {}", 5);
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
    

    let response = route_service::execute_request(request);

    let temp = construct_response_from(response);
    info!("{}", temp);

    write(stream, temp);
}


fn write(mut stream : TcpStream, response: String) {
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn construct_response_from(reponse : HTTPResponse) -> String {
    let status_line: String= construct_status_line(reponse.code);
    let mut body = reponse.body;

    let content = body.get_or_insert("".to_string());

    format!(
        "{}\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        "application/json",
        content.len(),
        content
    )
}

fn construct_status_line(code : i32) -> String {
    format!("{} {} {}", PROTOCOL, code, message_from_code(code))
}

fn message_from_code(code : i32) -> String {
    match code {
        200 =>  "OK".to_string(),
        500 => "INTERNAL".to_string(),
        404 => "NOT FOUND".to_string(),
        _  => "WTF".to_string()
    }
}