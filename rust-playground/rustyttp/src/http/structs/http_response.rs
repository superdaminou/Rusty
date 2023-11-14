use super::response::Response;
use std::fmt;

const PROTOCOL : &str= "HTTP/1.1";

pub struct HTTPResponse {
    code: i32,
    acces_control: String,
    content_type : String,
    body: String,
    headers: Vec<String>
}

impl HTTPResponse {
    fn new(code: i32,headers: Vec<String>,  body: Option<&str>) -> HTTPResponse {
        return HTTPResponse {code: code, headers: headers,  body: String::from(body.unwrap_or("")), acces_control: "*".to_string(), content_type: "application/json".to_string()};
    }    
}


impl From<Response> for HTTPResponse {
    fn from(response: Response) -> Self {
        HTTPResponse::new(response.code,Vec::new() ,response.body.as_deref())
    }
}

impl From<Vec<String>> for HTTPResponse {
    fn from(headers: Vec<String>) -> Self {
        return HTTPResponse::new(200, headers, None);
    }
}

fn construct_status_line(code : i32) -> String {
    format!("{} {} {}", PROTOCOL, code, message_from_code(code))
}

fn message_from_code(code : i32) -> String {
    match code {
        200 =>  String::from("OK"),
        500 => "INTERNAL".to_string(),
        404 => "NOT FOUND".to_string(),
        400 => "MALFORMED".to_string(),
        other  => "".to_string()
    }
}

impl fmt::Display for HTTPResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        return write!(f,
            "{}\r\n{}Access-Control-Allow-Origin: {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            construct_status_line(self.code),
            self.headers.join("\r\n"),
            self.acces_control,
            self.content_type,
            self.body.len(),
            self.body
        );
    }
}
