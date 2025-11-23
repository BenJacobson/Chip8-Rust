mod bit_manipulation;
mod display;
mod processor;
mod instructions;

pub use processor::Processor as Chip8;

#[cfg(test)]
mod test_processor;
