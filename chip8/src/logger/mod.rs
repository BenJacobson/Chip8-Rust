pub mod file_log_source;
pub mod null_log_source;

use file_log_source::FileLogSource;
use null_log_source::NullLogSource;

use std::env;
use std::fmt::Debug;
use std::fs::OpenOptions;

#[derive(Debug)]
pub struct Logger {
    log_source: Box<dyn LogSource>,
}

impl Logger {
    pub fn new(log_source: Box<dyn LogSource>) -> Self {
        Logger { log_source }
    }

    pub fn new_null_logger() -> Self {
        Logger {
            log_source: Box::new(NullLogSource::new()),
        }
    }

    pub fn log(&mut self, message: &str) {
        self.log_source.as_mut().write(message);
    }
}

pub trait LogSource: Debug + Send + Sync {
    fn write(&mut self, message: &str);
}

pub fn from_env_args() -> Logger {
    let log_file_path_option = env::args()
        .flat_map(|str| str.strip_prefix("--log_file=").map(|str| str.to_string()))
        .next();
    let log_source: Box<dyn LogSource> = match log_file_path_option {
        Some(log_file_path) => {
            let file_open_error_msg = format!("Failed to open log file: {}", log_file_path);
            let log_file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(log_file_path)
                .expect(file_open_error_msg.as_str());
            Box::new(FileLogSource::new(log_file))
        }
        None => Box::new(NullLogSource::new()),
    };
    Logger::new(log_source)
}

#[cfg(test)]
mod test_logger;
