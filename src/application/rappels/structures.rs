use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use tokio_postgres::Row;
use log::info;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Rappel {
    pub id: i32,
    pub nom: String,
    pub date_limite: NaiveDate,
    pub repetition: i32,
    pub criticite: String
}


impl From<&Row> for Rappel {
    fn from(row: &Row) -> Self {
        Rappel {
            id : row.get(0),
            nom: row.get(1),
            date_limite:  row.get("date_limite"),
            repetition: row.get("repetition"),
            criticite: row.get("criticite")
        }
    }
}


impl From<String> for Rappel {
    fn from(body: String) -> Self {
        info!("{}",body);
        return serde_json::from_str(&body).unwrap();
    }
}
