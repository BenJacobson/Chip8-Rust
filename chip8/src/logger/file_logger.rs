use super::Logger;

use std::env;
use std::fs::OpenOptions;
use std::io::Write;

pub fn from_env_args() -> Logger {
    let log_file_path_option = env::args()
        .flat_map(|str| str.strip_prefix("--log_file=").map(|str| str.to_string()))
        .next();
    let log_fn: Box<dyn FnMut(&str)> = match log_file_path_option {
        Some(log_file_path) => {
            let file_open_error_msg = format!("Failed to open log file: {}", log_file_path);
            let mut log_file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(log_file_path)
                .expect(file_open_error_msg.as_str());
            Box::new(move |str: &str| {
                let _ = writeln!(log_file, "{}", str);
            })
        }
        None => Box::new(|_str| ()),
    };
    Logger::new(log_fn)
}
