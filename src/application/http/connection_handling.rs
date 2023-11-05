use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use log::info;
use std::env;
use crate::application::routes::route_service;
use crate::application::http::structs::thread_pool::ThreadPool;
use crate::application::http::structs::http_response::HTTPResponse;


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
    
    let response = HTTPResponse::from(route_service::execute_request(request));

    info!("{}", response.to_string());

    write(stream, response.to_string());
}


fn write(mut stream : TcpStream, response: String) {
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}