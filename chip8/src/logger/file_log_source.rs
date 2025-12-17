use super::LogSource;

use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub struct FileLogSource {
    log_file: File,
}

impl FileLogSource {
    pub fn new(log_file: File) -> Self {
        FileLogSource { log_file }
    }
}

impl LogSource for FileLogSource {
    fn write(&mut self, message: &str) {
        let _ = writeln!(self.log_file, "{}", message);
    }
}
