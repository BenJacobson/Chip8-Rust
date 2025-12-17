use crate::logger::LogSource;

use super::Logger;

use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct TestLogSource {
    log_str: Arc<Mutex<String>>,
}

impl LogSource for TestLogSource {
    fn write(&mut self, message: &str) {
        *self.log_str.lock().unwrap() = message.to_string();
    }
}

#[test]
fn test_logger_logs() {
    let log_str = Arc::new(Mutex::new("".to_owned()));
    let test_log_source = TestLogSource {
        log_str: log_str.clone(),
    };

    let mut logger = Logger::new(Box::new(test_log_source));
    logger.log("Hello there friend");

    assert_eq!("Hello there friend", *log_str.lock().unwrap());
}
