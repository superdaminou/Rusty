mod rappels;
mod database_service;
mod route;

use notification_playground::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use rappels::Rappels;
use std::str;

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

    let request: &str = str::from_utf8(&buffer).unwrap();
    let request_lines : Vec<&str> = request.split("\r\n").collect();

    println!("{}", request);


    let (status_line , contents ) = match route::routes.contains(&request_lines[0]) {
        true => get_call(request_lines[0]),
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

fn get_call(route : &str) -> (&str, String) {
    if route.eq(route::get_all) {
        let rows = match rappels::get_all() {
            Ok(results) => results,
            Err(error) => panic!("Fatal: {}", error),
        };
        let rows_json = format!("{{\"result\":[{}]}}", rows.join(", "));
        return ("HTTP/1.1 200 OK", rows_json)
    } else if (route.eq(route::post_rappel)) {
        let rappel = Rappels { nom: "Yolo".to_string(), date_limite: "2021-02-03".to_string(), repetition: 1 ,criticite: "easy".to_string()}; 
            let updated_rows = match rappels::add_one(rappel) {
                Ok(result)  => result,
                Err(error) => panic!("Fatal: {}", error)
            };
            return ("HTTP/1.1 200 OK", updated_rows.to_string())
    } else {
        return ("", "".to_string());
    } 

}
