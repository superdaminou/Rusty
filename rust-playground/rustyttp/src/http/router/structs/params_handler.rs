use std::collections::HashMap;

use crate::{HTTPRequest, Route};


pub struct ParamsHandler {
    pub params: Params,
    pub body: Option<String>
}


pub type Params = HashMap<String, String>;

impl From<(HTTPRequest, Route)> for ParamsHandler {
    fn from((request , ressource): (HTTPRequest, Route)) -> Self {
        let positions =  ressource.route.split('/')
            .enumerate()
            .filter(|(_, val)| val.starts_with('{'))
            .collect::<HashMap<usize, &str>>();

        let params = request.start_line.ressource.split('/')
            .enumerate()
            .filter(|(index, _)| positions.keys().collect::<Vec<&usize>>().contains(&index))
            .map(|(index, param)| (positions.get(&index).unwrap().to_string(), param.to_string()))
            .collect::<HashMap<String, String>>();
        
        ParamsHandler { params, body: request.body }
    }
}