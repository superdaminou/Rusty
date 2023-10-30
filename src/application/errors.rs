use std::error::Error;
use std::fmt;
use serde_json::Error as SerdeError;

#[derive(Debug)]
pub struct TechnicalError {
    details: String
}

impl TechnicalError {
    pub fn new(msg: String) -> TechnicalError {
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

// a test function that returns our error result
fn raises_my_error(yes: bool) -> Result<(),TechnicalError> {
    if yes {
        Err(TechnicalError::new("borked".to_string()))
    } else {
        Ok(())
    }
}