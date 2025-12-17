use super::LogSource;

#[derive(Debug)]
pub struct NullLogSource {}

impl NullLogSource {
    pub fn new() -> Self {
        NullLogSource {}
    }
}

impl LogSource for NullLogSource {
    fn write(&mut self, _message: &str) {}
}
