use crate::application::rappels::rappels_controller;


pub const GET_ALL : (&str, &str) = ("GET","/all");
pub const POST_RAPPEL : (&str, &str)  = ("POST", "/rappel");
pub const GET_ONE_RAPPEL : (&str, &str) = ("GET", "/{id}");

pub const ROUTES : [(&str, &str); 3] = [GET_ALL, 
                                POST_RAPPEL,
                                GET_ONE_RAPPEL];

                    


pub fn execute(route : (&str, &str), body : Option<String>) -> (u16, Option<String>) { 
    return match route {   
        GET_ALL => rappels_controller::get_rappels(),
        POST_RAPPEL => rappels_controller::add_rappel(body),
        //GET_ONE_RAPPEL => rappels_controller::get_rappel(params.get(0).unwrap()),
        _ => (404, None)
    }
}