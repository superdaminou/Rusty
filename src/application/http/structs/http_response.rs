use crate::application::errors::TechnicalError;
use super::response::Response;
use std::fmt;

const PROTOCOL : &str= "HTTP/1.1";

pub struct HTTPResponse {
    pub code: i32,
    acces_control: String,
    pub content_type : String,
    pub body: String
}

impl HTTPResponse {
    fn new(code: i32, body: Option<String>) -> HTTPResponse {
        return HTTPResponse {code: code, body: body.unwrap_or("".to_string()), acces_control: "*".to_string(), content_type: "application/json".to_string()};
    }    
}


impl From<Response> for HTTPResponse {
    fn from(response: Response) -> Self {
        HTTPResponse::new(response.0.0, response.0.1)
    }
}

impl From<TechnicalError> for HTTPResponse {
    fn from(error: TechnicalError) -> Self {
        HTTPResponse::new(500, Some(error.to_string()))
    }
}


impl From<Result<Response, TechnicalError>> for HTTPResponse {
    fn from(result: Result<Response, TechnicalError>) -> Self {
        return HTTPResponse::from(result.unwrap_or_else(|err| Response((500, Some(err.to_string())))));
    }
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

impl fmt::Display for HTTPResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f,
            "{}\r\nAccess-Control-Allow-Origin: {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            construct_status_line(self.code),
            self.acces_control,
            self.content_type,
            self.body.len(),
            self.body
        );
    }
}
