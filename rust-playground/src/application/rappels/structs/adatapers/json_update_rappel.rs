use chrono::NaiveDate;
use log::info;
use serde::{Deserialize, Serialize};

use crate::application::rappels::structs::Rappel;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateRappel {
    pub id: i32,
    pub nom: String,
    pub date_limite: NaiveDate,
    pub repetition: i32,
    pub criticite: String
}



impl From<String> for UpdateRappel {
    fn from(body: String) -> Self {
        info!("Deserializing to Rappel: {}",body);
        return serde_json::from_str(&body).unwrap();
    }
}


impl From<UpdateRappel> for Rappel {
    fn from(rappel: UpdateRappel) -> Self {
        Rappel {id: Some(rappel.id), nom: String::from("Ok"), criticite: String::from("Ok"), repetition: 3, date_limite: NaiveDate::MIN}
    }
}

