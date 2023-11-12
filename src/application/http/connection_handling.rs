use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::panic::catch_unwind;
use std::str;
use log::info;
use std::env;
use crate::application::http::structs::thread_pool::ThreadPool;
use crate::application::http::structs::http_response::HTTPResponse;
use crate::application::http::structs::http_request::HTTPRequest;

use super::structs::response::Response;

pub fn open_connection(handler : fn(HTTPRequest) -> Response){
    info!("Opening connection and listening");

    let adresse = env::var("SERVER_ADRESS").unwrap_or("127.0.0.1:7878".to_string());
    info!("Start listening on {}", adresse);
    let listener = TcpListener::bind(adresse).unwrap();
    info!("Initializing thread pool : {}", 5);
    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(stream, handler);
        });
    }
}


// PRIVATE

// Router
fn handle_connection(mut stream: TcpStream, handler : fn(HTTPRequest) -> Response) {
    info!("Handling Connection");
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let response = str::from_utf8(&buffer)
        .map(HTTPRequest::from)
        .map(|request |
            catch_unwind(|| (handler)(HTTPRequest::from(request)))
            .unwrap_or_else(|err| Response((500, Some(String::from("Internal Error"))))))
        .map(|response| HTTPResponse::from(response))
        .unwrap_or_else(|err| HTTPResponse::from(Response((500, Some(String::from("Parsing error"))))));
    

    info!("{}", response.to_string());
    write(stream, response.to_string());
}


fn write(mut stream : TcpStream, response: String) {
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}