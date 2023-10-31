use std::error::Error;
use std::fmt;
use serde_json::Error as SerdeError;
use tokio_postgres::Error as PostgresError;

#[derive(Debug)]
pub struct TechnicalError {
    details: String
}

impl TechnicalError {
    fn new(msg: String) -> TechnicalError {
        TechnicalError{details: msg}
    }
}

impl fmt::Display for TechnicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for TechnicalError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<SerdeError> for TechnicalError {
    fn from(err: SerdeError) -> Self {
        TechnicalError::new(err.to_string())
    }
}

impl From<PostgresError> for TechnicalError {
    fn from(err: PostgresError) -> Self {
        TechnicalError::new(err.to_string())
    }
}

impl From<String> for TechnicalError {
    fn from(err: String) -> Self {
        TechnicalError::new(err)
    }
}