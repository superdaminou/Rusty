use chrono::NaiveDate;
use log::info;
use serde::{Deserialize, Serialize};

use crate::application::{rappels::structs::Rappel, errors::TechnicalError};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateRappel {
    pub id: i32,
    pub nom: String,
    pub date_limite: NaiveDate,
    pub repetition: i32,
    pub criticite: String
}


impl UpdateRappel {
    pub fn extract(body: String) -> Result<UpdateRappel, TechnicalError> {
        info!("Deserializing to Rappel: {}",body);
        return serde_json::from_str(&body).map_err(|err| TechnicalError::from(err));
    }
}


impl From<UpdateRappel> for Rappel {
    fn from(rappel: UpdateRappel) -> Self {
        Rappel {id: Some(rappel.id), nom: rappel.nom, criticite: rappel.criticite, repetition: rappel.repetition, date_limite: rappel.date_limite}
    }
}

