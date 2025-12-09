pub mod file_logger;

use std::fmt;

pub struct Logger {
    log_fn: Box<dyn FnMut(&str)>,
}

impl Logger {
    pub fn new(log_fn: Box<dyn FnMut(&str)>) -> Logger {
        Self { log_fn }
    }

    pub fn log(&mut self, str: &str) {
        (*self.log_fn)(str);
    }
}

impl fmt::Debug for Logger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Logger").finish()
    }
}

#[cfg(test)]
mod test_logger;
