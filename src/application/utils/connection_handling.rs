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
    
    let (status_line , contents ) = route::execute_request(request_lines);

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