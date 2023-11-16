use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::panic::catch_unwind;
use std::str;
use log::info;
use std::env;
use crate::http::errors::internal::InternalError;
use crate::http::errors::malformed::MalformedError;
use crate::http::structs::thread_pool::ThreadPool;
use crate::http::structs::http_response::HTTPResponse;
use crate::http::structs::http_request::HTTPRequest;

use super::HttpVerb;
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

    let response = 
        str::from_utf8(&buffer)
        .map_err(|error| 
            MalformedError::from(error))
        .map(|request| 
            HTTPRequest::request_from(request))
        .map(|request | 
            request.map(|request| 
                handle_request(request, handler))
                .unwrap_or_else(|error| HTTPResponse::from(error)))
        .unwrap_or_else(|err| HTTPResponse::from(err));

            
    info!("{}", response.to_string());
    write(stream, response.to_string());
}


fn write(mut stream : TcpStream, response: String) {
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_request(request: HTTPRequest, handler : fn(HTTPRequest) -> Response) -> HTTPResponse {
    catch_unwind(|| {
        match request.verb {
            HttpVerb::OPTION => options(),
            _ => HTTPResponse::from((handler)(request))

        }
    })
    .map_err(|_| InternalError::from("Internal Server Error"))
    .unwrap_or_else(|err| HTTPResponse::from(err))
}

fn options() -> HTTPResponse {
    let headers = vec!["Access-Control-Allow-Methods: POST, GET, DELETE, PATCH, OPTIONS\r\n".to_string()];
    HTTPResponse::from(headers)
}