mod rappels;

use notification_playground::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

mod database_service;

// Listen on port 7878 and for each stream incoming create a new thread.
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

// Router
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let get_all = b"GET /all HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let post_rappel = b"POST /rappel HTTP/1.1\r\n";


    // TODO replace with match pattern
    let (status_line, contents) = if buffer.starts_with(get_all) {
        let rows = match rappels::get_all() {
            Ok(results) => results,
            Err(error) => panic!("Fatal: {}", error)
        };
        let rows_json = format!("{{\"result\":[{}]}}", rows.join(", "));
        ("HTTP/1.1 200 OK", rows_json)
    } else if buffer.starts_with(post_rappel) {
        

        ("HTTP/1.1 200 OK", "hello.html".to_string())
    } else {
        ("HTTP/1.1 404 NOT FOUND", "Not found".to_string())
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

