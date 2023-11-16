pub struct Response {
    pub code: i32,
    pub headers: Vec<String>,
    pub body: Option<String>
}


impl Response {
    fn new(code: i32,headers: Vec<String>,  body: Option<String>) -> Response {
        return Response {code: code, headers: headers, body: body};
    }    
}


impl From<i32> for Response {
    fn from(code: i32) -> Self {
        return Response::new(code, Vec::new(), None);
    }
}


impl From<(i32, &str)> for Response {
    fn from(code: (i32, &str)) -> Self {
        return Response::new(code.0, Vec::new(),Some(code.1.to_string()));
    }
}

impl From<(i32, String)> for Response {
    fn from(code: (i32, String)) -> Self {
        return Response::new(code.0, Vec::new(),Some(code.1));
    }
}

