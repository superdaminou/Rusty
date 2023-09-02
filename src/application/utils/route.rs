use crate::application::rappels::rappels_controller;
use log::info;

pub const GET_ALL : (&str, &str) = ("GET","/all");
pub const POST_RAPPEL : (&str, &str)  = ("POST", "/rappel");
pub const GET_ONE_RAPPEL : (&str, &str) = ("GET", "/{id}");

pub const ROUTES : [(&str, &str); 3] = [GET_ALL, 
                                POST_RAPPEL,
                                GET_ONE_RAPPEL];


macro_rules! call (
    ($f: expr, $($params:tt)*) => {
        make_call!($f, () $($params)*)
    };
); 

macro_rules! make_call {
    ($f: expr, ($($args:tt)*)) => { $f($($args)*) };
    ($f: expr, () I $($params:tt)*) => {
        make_call!($f, (2) $($params)*)
    };
    ($f: expr, ($($args:tt)*) I $($params:tt)*) => {
        make_call!($f, ($($args)*, 5) $($params)*)
    };
}


pub fn execute(route : (&str, &str), params : Vec<String>, body : Option<String>) -> (u16, Option<String>) { 
    return match route {   
        GET_ALL => rappels_controller::get_rappels(),
        POST_RAPPEL => rappels_controller::add_rappel(body),
        GET_ONE_RAPPEL => rappels_controller::get_rappel(params.get(0).unwrap().to_owned()),
        _ => (404, None)
    }
}


fn foo(a: i32, b: i32) {
    info!("OUAAAI {} et {}", a, b); 
}