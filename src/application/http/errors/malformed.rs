use core::fmt;
use std::{error::Error, str::Utf8Error};

use crate::application::http::structs::{http_response::HTTPResponse, response::Response};


#[derive(Debug)]
pub struct MalformedError  {
    details: String
}

impl MalformedError {
    fn new(msg: String) -> MalformedError {
        MalformedError{details: msg}
    }
}

impl fmt::Display for MalformedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MalformedError {
    fn description(&self) -> &str {
        &self.details
    }
}


impl From<&str> for MalformedError {
    fn from(err: &str) -> Self {
        MalformedError::new(err.to_string())
    }
}


impl From<Utf8Error> for MalformedError {
    fn from(err: Utf8Error) -> Self {
        MalformedError::new(err.to_string())
    }
}

impl From<MalformedError> for HTTPResponse {
    fn from(_: MalformedError) -> Self {
        return HTTPResponse::from(Response((400, Some(String::from("Malformed Url")))));
    }
}
