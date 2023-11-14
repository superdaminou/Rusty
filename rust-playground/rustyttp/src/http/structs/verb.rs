use std::str::FromStr;

use strum_macros::Display;

#[derive(PartialEq, Eq, Display)]
pub enum HttpVerb {
    POST,
    GET,
    PUT,
    DELETE,
    PATCH,
    OPTION
}

impl FromStr for HttpVerb {
    type Err = ();

    fn from_str(input: &str) -> Result<HttpVerb, ()> {
        return match input {
            "POST" => Ok(Self::POST),
            "GET" => Ok(Self::GET),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "PATCH" => Ok(Self::PATCH),
            "OPTIONS" => Ok(Self::OPTION),
            _ =>  panic!("Could not find verb")
        }
    }
}