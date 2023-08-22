
#[cfg(test)]

mod tests {

    use crate::application::utils::route_utils;

    #[test]
    fn compare_route_ok() {
        assert_eq!((200, Some("e".to_string())), route_utils::execute_request(vec!["/rappel"]));
    }

    #[test]
    fn compare_route_ko() {
        assert_eq!((200, Some("e".to_string())), route_utils::execute_request(vec!["/rappel"]))
    }

    #[test]
    fn compare_route_ko_id_missing() {
        assert_eq!((200, Some("e".to_string())), route_utils::execute_request(vec!["/rappel"]))
    }
}