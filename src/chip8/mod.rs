mod bit_manipulation;
mod display;
mod processor;
mod instructions;

pub use processor::Processor as Chip8;

#[cfg(test)]
mod test_display;

#[cfg(test)]
mod test_instructions;

#[cfg(test)]
mod test_execute_digits;

#[cfg(test)]
mod test_execute_flags;

#[cfg(test)]
mod test_processor;
