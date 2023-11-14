use chrono::NaiveDate;
use log::info;
use serde::{Deserialize, Serialize};

use crate::application::rappels::structs::Rappel;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WriteRappel {
    pub nom: String,
    pub date_limite: NaiveDate,
    pub repetition: i32,
    pub criticite: String
}



impl From<String> for WriteRappel {
    fn from(body: String) -> Self {
        info!("Deserializing to Rappel: {}",body);
        return serde_json::from_str(&body).unwrap();
    }
}

impl From<WriteRappel> for Rappel {
    fn from(rappel: WriteRappel) -> Self {
        Rappel {id: None, nom: rappel.nom, criticite: rappel.criticite, repetition: rappel.repetition, date_limite: NaiveDate::from(rappel.date_limite)}
    }
}



