use super::processor::*;
use crate::logger::Logger;

#[test]
#[should_panic]
fn test_uninitialized_execution_panics() {
    let logger = Logger::new(Box::new(|_| ()));
    let mut processor = Processor::new(logger);
    while !processor.run_next_instruction() {}
}
