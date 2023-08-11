use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Rappel {
    pub nom: String,
    pub date_limite: String,
    pub repetition: i32,
    pub criticite: String
}
