

use super::rappels::rappels_controller;
use rustyttp::Route;
use rustyttp::Verb;

pub fn routes() -> Vec<Route> {
    let routes = vec![
        Route {verb: Verb::GET,   route: String::from("/rappels"), method: rappels_controller::get_rappels},
        Route {verb: Verb::GET, route: "/rappel/{id}".to_string(),method: rappels_controller::get_rappel},
        Route {verb: Verb::POST,   route: "/rappel".to_string(), method: rappels_controller::add_rappel},
        Route {verb: Verb::PUT,   route: "/rappel/{id}".to_string(), method: rappels_controller::update_rappel},
        Route {verb: Verb::DELETE,   route: "/rappel/{id}".to_string(), method: rappels_controller::delete_rappel},
        ];

    return routes;
}