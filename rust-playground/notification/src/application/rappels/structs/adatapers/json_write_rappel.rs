use chrono::NaiveDate;
use log::info;
use serde::{Deserialize, Serialize};

use crate::application::{rappels::structs::Rappel, errors::TechnicalError};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WriteRappel {
    pub nom: String,
    pub date_limite: NaiveDate,
    pub repetition: i32,
    pub criticite: String
}



impl WriteRappel {
    pub fn extract(body: String) -> Result<WriteRappel, TechnicalError> {
        info!("Deserializing to Rappel: {}",body);
        return serde_json::from_str(&body).map_err(|err| TechnicalError::from(err));
    }
}

impl From<WriteRappel> for Rappel {
    fn from(rappel: WriteRappel) -> Self {
        Rappel {id: None, nom: rappel.nom, criticite: rappel.criticite, repetition: rappel.repetition, date_limite: NaiveDate::from(rappel.date_limite)}
    }
}




