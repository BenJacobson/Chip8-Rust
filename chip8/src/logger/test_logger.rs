use super::Logger;

use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_logger_logs() {
    let log_str = Rc::new(RefCell::new("".to_owned()));
    let log_str_closure = log_str.clone();
    let log_fn = move |str: &str| {
        log_str_closure.replace(str.to_owned());
    };

    let mut logger = Logger::new(Box::new(log_fn));
    logger.log("Hello there friend");

    assert_eq!("Hello there friend", log_str.borrow().as_str());
}
