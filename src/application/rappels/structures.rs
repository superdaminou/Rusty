use serde::{Deserialize, Serialize};
use chrono::{NaiveDate};

#[derive(Debug, Deserialize, Serialize)]
pub struct Rappel {
    pub nom: String,
    pub date_limite: NaiveDate,
    pub repetition: i32,
    pub criticite: String
}
