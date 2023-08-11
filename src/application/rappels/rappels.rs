use log::info;
use crate::application::database::rappel_db_service;
use crate::application::rappels::Rappel;


pub fn get_rappels() -> String {
    let rows = match rappel_db_service::get_all() {
        Ok(results) => results,
        Err(error) => panic!("Fatal: {}", error),
    };
    return format!("{{\"result\":[{}]}}", rows.join(", "));
}


pub fn add_rappel(body : Option<String>) -> (&'static str, String) {

    let body: String = match body {
        Some(result) => result,
        None => return ("HTTP/1.1 403 Forbiden", "Body mandatory".to_string())
    };
    

    info!("Trying to deserialize : {}", body);
    let rappel: Rappel = match serde_json::from_str(&body) {
        Ok(rappel) => rappel,
        Err(err) => return ("HTTP/1.1 500 KO", format!("Errors while parsing json {} stacktrace {}",&body, err.to_string()))
    };

    return match rappel_db_service::add_one(rappel) {
        Ok(result) => ("HTTP/1.1 200 OK", result.to_string()),
        Err(error)=> ("HTTP/1.1 500 Server Error", error.to_string())
    };
}